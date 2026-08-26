use crate::prelude::*;
use tokio::sync::broadcast::*;

#[derive(Debug, Clone)]
pub struct Xanterella {
    pub path: String,
    pub home: String,
    pub fast: bool,
    pub debug: bool,
    pub automate: bool,
    pub sender: Option<Sender<EventFormat>>,
}

impl PartialEq for Xanterella {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path &&
        self.home == other.home &&
        self.fast == other.fast &&
        self.debug == other.debug &&
        self.automate == other.automate
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventFormat {
    pub state: EventState,
    pub step: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventState {
    Run,
    Finish, 
    Failed,
}

impl Xanterella {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Xanterella {
            path: String::new(),
            home: String::new(),
            fast: false,
            debug: false,
            automate: false,
            sender: None,
        }
    }

    pub fn set_path(&mut self, value: &str) {
        self.path = value.to_string();
    }

    pub fn set_home(&mut self, value: &str) {
        self.home = value.to_string();
    }

    pub fn set_fast(&mut self, value: bool) {
        self.fast = value;
    }

    pub fn set_debug(&mut self, value: bool) {
        self.debug = value;
    }

    pub fn set_automate(&mut self, value: bool) {
        self.automate = value;
    }

    pub fn set_sender(&mut self, value: Sender<EventFormat>) {
        self.sender = Some(value);
    }

    pub fn log_event(&mut self, event: Events) {
        let (state, name): (EventState, &str) = match event {
            // Utils
            // utils/Git.rs
            Events::RunGitCommit => (EventState::Run, "Git Commit"),
            Events::RunGitCheckout => (EventState::Run, "Git Checkout"),
            Events::RunGitCheckoutCreate => (EventState::Run, "Git Checkout(Create)"),
            Events::RunGitMerge => (EventState::Run, "Git Merge"),
            Events::RunGitPr => (EventState::Run, "Git Pull Request"),

            Events::OkGitCommit => (EventState::Finish, "Git Commit"),
            Events::OkGitCheckout => (EventState::Finish, "Git Checkout"),
            Events::OkGitCheckoutCreate => (EventState::Finish, "Git Checkout(Create)"),
            Events::OkGitMerge => (EventState::Finish, "Git Merge"),
            Events::OkGitPr => (EventState::Finish, "Git Pull Request"),
            // utils/Check.rs
            Events::RunCheckNix => (EventState::Run, "Nix Flake Check"),

            Events::OkCheckNix => (EventState::Finish, "Nix Flake Check"),
            // utils/Config.rs
            Events::RunConfigCreateDir => (EventState::Run, "Create Config Dir"),
            Events::RunConfigGenBasic => (EventState::Run, "Create basic Config"),

            Events::OkConfigCreateDir => (EventState::Finish, "Create Config Dir"),
            Events::OkConfigGenBasic => (EventState::Finish, "Create basic Config"),
            // Installer
            // installer/Core.rs
            Events::RunRemoteIntegration => (EventState::Run, "Remote Integration"),
            Events::RunRemotePrepFs => (EventState::Run, "Remote Preperation of Fs"),
            Events::RunRemoteInstall => (EventState::Run, "Remote Install"),
            Events::RunRemoteInstallCleanup => (EventState::Run, "Remote Cleanup"),

            Events::OkRemoteIntegration => (EventState::Finish, "Remote Integration"),
            Events::OkRemotePrepFs => (EventState::Finish, "Remote Preperation of Fs"),
            Events::OkRemoteInstall => (EventState::Finish, "Remote Install"),
            Events::OkRemoteInstallCleanup => (EventState::Finish, "Remote Cleanup"),
            // installer/Ping.rs
            Events::RunPing => (EventState::Run, "Ping"),
            Events::RunPingSsh => (EventState::Run, "SSH Ping"),

            Events::OkPing => (EventState::Finish, "Ping"),
            Events::OkPingSsh => (EventState::Finish, "SSH Ping"),
            // installer/Deploy.rs
            Events::RunNixBuild => (EventState::Run, "Nix Build"),
            Events::RunNixCopy => (EventState::Run, "Nix Copy"),
            Events::RunCreateProfile => (EventState::Run, "Create Profile"),
            Events::RunPrepSys => (EventState::Run, "Preperation of Sys"),
            Events::RunActivateSys => (EventState::Run, "Activate Sys"),
            Events::RunActivateBootloader => (EventState::Run, "Activate Bootloader"),
            Events::RunRebootSys => (EventState::Run, "Reboot"),

            Events::OkNixBuild => (EventState::Finish, "Nix Build"),
            Events::OkNixCopy => (EventState::Finish, "Nix Copy"),
            Events::OkCreateProfile => (EventState::Finish, "Create Profile"),
            Events::OkPrepSys => (EventState::Finish, "Preperation of Sys"),
            Events::OkActivateSys => (EventState::Finish, "Activate Sys"),
            Events::OkActivateBootloader => (EventState::Finish, "Activate Bootloader"),
            Events::OkRebootSys => (EventState::Finish, "Reboot"),
            // installer/Drives.rs
            Events::RunPartEfi => (EventState::Run, "Partitionate Efi"),
            Events::RunPartRoot => (EventState::Run, "Partitionate Root"),
            Events::RunFormatEfi => (EventState::Run, "Format Efi"),
            Events::RunFormatRoot => (EventState::Run, "Format Root"),
            Events::RunCreateBootDir => (EventState::Run, "Create Boot Dir"),
            Events::RunMountBoot => (EventState::Run, "Mount Boot"),
            Events::RunMountRoot => (EventState::Run, "Mount Root"),

            Events::OkPartEfi => (EventState::Finish, "Partitionate Efi"),
            Events::OkPartRoot => (EventState::Finish, "Partitionate Root"),
            Events::OkFormatEfi => (EventState::Finish, "Format Efi"),
            Events::OkFormatRoot => (EventState::Finish, "Format Root"),
            Events::OkCreateBootDir => (EventState::Finish, "Create Boot Dir"),
            Events::OkMountBoot => (EventState::Finish, "Mount Boot"),
            Events::OkMountRoot => (EventState::Finish, "Mount Root"),
            // installer/Helper.rs
            Events::RunGetHardware => (EventState::Run, "Get Hardware"),
            Events::RunGetDrives => (EventState::Run, "Get Drives"),

            Events::OkGetHardware => (EventState::Finish, "Get Hardware"),
            Events::OkGetDrives => (EventState::Finish, "Get Drives"),
            // installer/Inject.rs
            Events::RunInjectTailscale => (EventState::Run, "Inject Tailscale"),
            Events::RunInjectWifi => (EventState::Run, "Inject Wifi"),

            Events::OkInjectTailscale => (EventState::Finish, "Inject Tailscale"),
            Events::OkInjectWifi => (EventState::Finish, "Inject Wifi"),
        };

        if let Some(ref tx) = self.sender {
            let _ = tx.send(EventFormat {
                state,
                step: name.to_string(),
            });
        }
    }
}

#[cfg(test)]
#[path = "xanterella_test.rs"]
mod tests;
