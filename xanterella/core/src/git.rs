use std::fs::create_dir_all;

use crate::prelude::*;

pub struct Initial;
pub struct Initialized;
pub struct CheckedOut;
pub struct Merged;
pub struct Tracked;
pub struct Commited;
pub struct Pushed;

pub enum Branches {
    Main,
    Xanterella,
}

pub struct GitManager<State> {
    pub repo: String,
    pub directory: String,
    pub main_br: String,
    pub state: State,
}

impl GitManager<Initial> {
    pub async fn new(repo: String, directory: String, main_br: String) -> Self {
        Self {
            repo,
            directory,
            main_br,
            state: Initial,
        }
    }

    pub async fn init(self) -> Result<GitManager<Initialized>, EventsFailed> {
        let git_dir = PathBuf::from(&self.directory).join(".git");

        if !git_dir.exists() {
            eprintln!("Kein Git-Repo gefunden. Repo wird initialisiert");
            create_dir_all(&self.directory).map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            let init = Command::new("git")
                .arg("init")
                .current_dir(&self.directory)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !init.status.success() {
                return Err(EventsFailed::GitInit(String::from_utf8_lossy(&init.stderr).to_string()));
            }

            if !&self.repo.is_empty() {
                let remote = Command::new("git")
                    .args(["remote", "add", "origin", &self.repo])
                    .current_dir(&self.directory)
                    .output()
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !remote.status.success() {
                    return Err(EventsFailed::GitRemote(String::from_utf8_lossy(&remote.stderr).to_string()));
                }
            } else {
                eprintln!("Kein Remote Repo angegeben, verbinde nicht");
            }
        }
        if !&self.repo.is_empty() {
            let pull = Command::new("git")
                .args(["pull", "origin", &self.main_br])
                .current_dir(&self.directory)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !pull.status.success() {
                return Err(EventsFailed::GitPull(String::from_utf8_lossy(&pull.stderr).to_string()));
            }
        }

        if !&self.repo.is_empty() {
            let fetch = Command::new("git")
                .args(["fetch"])
                .current_dir(&self.directory)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !fetch.status.success() {
                return Err(EventsFailed::GitPull(String::from_utf8_lossy(&fetch.stderr).to_string()));
            }
        }

        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Initialized,
        })
    }
}

impl GitManager<Initialized> {
    pub async fn checkout(self, branch: Branches) -> Result<GitManager<CheckedOut>, EventsFailed> {
        let br_name = match branch {
            Branches::Main => &self.main_br,
            Branches::Xanterella => "xanterella",
        };

        let checkout = Command::new("git")
            .args(["checkout", br_name])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !checkout.status.success() {
            let create = Command::new("git")
                .args(["checkout", "-b", br_name])
                .current_dir(&self.directory)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;
            if !create.status.success() {
                return Err(EventsFailed::GitCheckoutCreate(String::from_utf8_lossy(&create.stderr).to_string()));
            }
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: CheckedOut,
        })
    }
}

impl GitManager<CheckedOut> {
    pub async fn merge(self) -> Result<GitManager<Merged>, EventsFailed> {
        let remote_branch = format!("origin/{}", &self.main_br);
        let merge = Command::new("git")
            .args(["merge", "--ff-only", "-n", &remote_branch])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !merge.status.success() {
            return Err(EventsFailed::GitMerge(String::from_utf8_lossy(&merge.stderr).to_string()));
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Merged,
        })
    }
}

impl GitManager<Merged> {
    pub async fn add(self) -> Result<GitManager<Tracked>, EventsFailed> {
        let add = Command::new("git")
            .args(["add", "."])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !add.status.success() {
            return Err(EventsFailed::GitAdd(String::from_utf8_lossy(&add.stderr).to_string()));
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Tracked,
        })
    }
    pub async fn reset(self, head: i8) -> Result<GitManager<Initialized>, EventsFailed> {
        let commit = format!("HEAD~{}", head);

        let reset = Command::new("git")
            .args(["reset", "--hard", &commit])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !reset.status.success() {
            return Err(EventsFailed::GitReset(String::from_utf8_lossy(&reset.stderr).to_string()));
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Initialized,
        })
    }
}

impl GitManager<Tracked> {
    pub async fn commit(self, msg: String) -> Result<GitManager<Commited>, EventsFailed> {
        let commit = Command::new("git")
            .args(["commit", "-m", &msg])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !commit.status.success() {
            return Err(EventsFailed::GitCommit(String::from_utf8_lossy(&commit.stderr).to_string()));
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Commited,
        })
    }
}

impl GitManager<Commited> {
    pub async fn push(self) -> Result<GitManager<Pushed>, EventsFailed> {
        let push = Command::new("git")
            .args(["push", "-u", "origin", "HEAD"])
            .current_dir(&self.directory)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !push.status.success() {
            return Err(EventsFailed::GitPush(String::from_utf8_lossy(&push.stderr).to_string()));
        }
        Ok(GitManager {
            repo: self.repo.clone(),
            directory: self.directory.clone(),
            main_br: self.main_br,
            state: Pushed,
        })
    }
}
