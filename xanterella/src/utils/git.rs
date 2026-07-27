use crate::prelude::*;

use crate::utils::get::*;

pub enum Branches {
    Main,
    Xanterella,
}
    
pub enum PrType {
    AddHost(String),
    RemoveHost(String),

    Changes(String),
}

pub trait Git {
    fn git_commit(&self, msg: &str) -> Result<(), EventsFailed>;
    fn git_checkout(&self, branch: Branches) -> Result<(), EventsFailed>;
    fn git_merge(&self, branch: Branches) -> Result<(), EventsFailed>;
    fn git_pr(&self, pr: PrType) -> Result<(), EventsFailed>;
}

impl Git for Xanterella {
    fn git_commit(&self, msg: &str) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitCommit(msg.clone()));

        let cmd = Command::new("git")
            .args(["commit", "-am", &msg])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GitCommit);
        };

        self.log_event(Events::OkGitCommit(msg.clone()));
        Ok(())
    }

    fn git_checkout(&self, branch: Branches) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitCheckout(branch));

        let br_name = match branch {
            Branches::Main => "main",
            Branches::Xanterella => "xanterella",
        };

        let cmd = Command::new("git")
            .args(["checkout", br_name])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            self.log_event(Events::RungGitCheckoutCreate(branch));

            let create = Command::new("git")
                .args(["checkout", "-b", br_name])
                .current_dir(self.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !create.status.success() {
                return Err(EventsFailed::GitCheckout);
            };

            self.log_event(Events::OkgGitCheckoutCreate(branch));
        };

        self.log_event(Events::RunGitCheckout(branch));
        Ok(())
    }

    fn git_merge(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitMerge);

        self.git_checkout(Branches::Xanterella)?;

        let cmd = Command::new("git")
            .args(["merge", "--ff-only", "-n", "main"])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GitMerge);
        }

        self.log_event(Events::OkGitMerge);
    }

    fn git_pr(&self, pr: PrType) -> Result<(), EventsFailed> {
        self.log_event(Events::RunGitPr(pr));

        let extra_part = "This is an automatic generated Pull Request";
        let title = match pr {
            PrType::AddHost(host) => format!("Xanterella: Add Host: {}", host),
            PrType::RemoveHost(host) => format!("Xanterella: Remove Host: {}", host),
            PrType::Changes(changes) => format!("Xanterella: Change Configs: {}", changes),
        };
        let body = match pr {
            PrType::AddHost(host) => format!("Xanterella added a Host \nAdded Host: {} \n{}", host, extra_part),
            PrType::RemoveHost(host) => format!("Xanterella removed a Host \nRemoved Host: {} \n{}", host, extra_part),
            PrType::Changes(changes) => format!("Xanterella changed the Configs \nChanges: {} \n{}", changes, extra_part),
        };

        let cmd = Command::new("gh")
            .args(["pr", "create", "--no-maintainer-edit"])
            .args(["-B", "main"])
            .args(["-t", &title])
            .args(["-b", &body])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GitPr);
        };

        self.log_event(Events::OkGitPr(pr));
        Ok(())
    }
}
