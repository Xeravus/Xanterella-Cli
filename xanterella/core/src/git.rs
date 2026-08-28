use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Branches {
    Main,
    Xanterella,
}

#[derive(Debug, Clone)]
pub enum PrType {
    AddHost(String),
    RemoveHost(String),

    Changes(String),
}

pub trait Git {
    fn git_commit(&mut self, msg: &str) -> Result<(), EventsFailed>;
    fn git_checkout(&mut self, branch: Branches) -> Result<(), EventsFailed>;
    fn git_merge(&mut self) -> Result<(), EventsFailed>;
    fn git_pr(&mut self, pr: PrType) -> Result<(), EventsFailed>;
    fn git_rollback(&mut self, head: i8) -> Result<(), EventsFailed>;
    fn git_reset(&mut self, head: i8) -> Result<(), EventsFailed>;
}

impl Git for Xanterella {
    fn git_commit(&mut self, msg: &str) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitCommit);

        if !self.debug {
            let cmd = Command::new("git")
                .args(["commit", "-am", msg])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::GitCommit(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.log_event(Events::OkGitCommit);
        Ok(())
    }

    fn git_checkout(&mut self, branch: Branches) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitCheckout);

        let br_name = match branch {
            Branches::Main => "main",
            Branches::Xanterella => "xanterella",
        };

        if !self.debug {
            let cmd = Command::new("git")
                .args(["checkout", br_name])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                self.log_event(Events::RunGitCheckoutCreate);

                let create = Command::new("git")
                    .args(["checkout", "-b", br_name])
                    .current_dir(self.get_path(Paths::Nixconf))
                    .output()
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !create.status.success() {
                    return Err(EventsFailed::GitCheckout(String::from_utf8_lossy(&create.stderr).to_string()));
                };

                self.log_event(Events::OkGitCheckoutCreate);
            };
        };

        self.log_event(Events::RunGitCheckout);
        Ok(())
    }

    fn git_merge(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitMerge);

        if !self.debug {
            self.git_checkout(Branches::Xanterella)?;

            let cmd = Command::new("git")
                .args(["merge", "--ff-only", "-n", "main"])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::GitMerge(String::from_utf8_lossy(&cmd.stderr).to_string()));
            }
        };

        self.log_event(Events::OkGitMerge);
        Ok(())
    }

    fn git_pr(&mut self, pr: PrType) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitPr);

        let extra_part = "This is an automatic generated Pull Request";
        let title = match pr {
            PrType::AddHost(ref host) => format!("Xanterella: Add Host: {}", host),
            PrType::RemoveHost(ref host) => format!("Xanterella: Remove Host: {}", host),
            PrType::Changes(ref changes) => format!("Xanterella: Change Configs: {}", changes),
        };
        let body = match pr {
            PrType::AddHost(ref host) => format!("Xanterella added a Host \nAdded Host: {} \n{}", host, extra_part),
            PrType::RemoveHost(ref host) => {
                format!("Xanterella removed a Host \nRemoved Host: {} \n{}", host, extra_part)
            }
            PrType::Changes(ref changes) => {
                format!("Xanterella changed the Configs \nChanges: {} \n{}", changes, extra_part)
            }
        };

        if !self.debug {
            let cmd = Command::new("gh")
                .args(["pr", "create", "--no-maintainer-edit"])
                .args(["-B", "main"])
                .args(["-t", &title])
                .args(["-b", &body])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::GitPr(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.log_event(Events::OkGitPr);
        Ok(())
    }

    fn git_rollback(&mut self, head: i8) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitRollback);

        self.git_reset(head)?;
        self.git_checkout(Branches::Main)?;

        self.log_event(Events::OkGitRollback);
        Ok(())
    }

    fn git_reset(&mut self, head: i8) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitReset);

        let commit = format!("HEAD~{}", head);

        if !self.debug {
            let cmd = Command::new("git")
                .args(["reset", "--hard", &commit])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::GitReset(String::from_utf8_lossy(&cmd.stderr).to_string()));
            }
        }
        self.log_event(Events::OkGitReset);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_git_debug() -> Xanterella {
        let mut xanterella = Xanterella::new();
        xanterella.debug = true;
        xanterella
    }

    #[test]
    fn test_utils_git_git_commit() {
        let result1 = test_git_debug().git_commit("Test");
        assert!(result1.is_ok());
    }

    #[test]
    fn test_utils_git_git_checkout() {
        let result1 = test_git_debug().git_checkout(Branches::Main);
        assert!(result1.is_ok());
    }

    #[test]
    fn test_utils_git_git_merge() {
        let result1 = test_git_debug().git_merge();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_utils_git_git_pr() {
        let result1 = test_git_debug().git_merge();
        assert!(result1.is_ok());
    }
}
