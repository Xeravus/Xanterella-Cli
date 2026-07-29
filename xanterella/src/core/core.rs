use crate::prelude::*;

pub struct Xanterella {
    pub path: String,
    pub home: String,
    pub fast: bool,
    pub debug: bool,
    pub automate: bool,
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

    pub fn log_event(&mut self, event: Events) {
        let (run, name): (bool, &str) = match event {
            // Utils
            // utils/Git.rs
            Events::RunGitCommit            => (true, "Git Commit"),
            Events::RunGitCheckout          => (true, "Git Checkout"),
            Events::RunGitCheckoutCreate    => (true, "Git Checkout(Create)"),
            Events::RunGitMerge             => (true, "Git Merge"),
            Events::RunGitPr                => (true, "Git Pull Request"),

            Events::OkGitCommit             => (false, "Git Commit"),
            Events::OkGitCheckout           => (false, "Git Checkout"),
            Events::OkGitCheckoutCreate     => (false, "Git Checkout(Create)"),
            Events::OkGitMerge              => (false, "Git Merge"),
            Events::OkGitPr                 => (false, "Git Pull Request"),
            // utils/Check.rs
            Events::RunCheckNix             => (true, "Nix Flake Check"),

            Events::OkCheckNix              => (false, "Nix Flake Check"),
            // utils/Config.rs
            Events::RunConfigCreateDir      => (true, "Create Config Dir"),
            Events::RunConfigGenBasic       => (true, "Create basic Config"),

            Events::OkConfigCreateDir       => (false, "Create Config Dir"),
            Events::OkConfigGenBasic        => (false, "Create basic Config"),
            // Installer
            // installer/Core.rs
            Events::RunRemoteIntegration    => (true, "Remote Integration"),
            Events::RunRemotePrepFs         => (true, "Remote Preperation of Fs"),
            Events::RunRemoteInstall        => (true, "Remote Install"),
            Events::RunRemoteInstallCleanup => (true, "Remote Cleanup"),

            Events::OkRemoteIntegration     => (false, "Remote Integration"),
            Events::OkRemotePrepFs          => (false, "Remote Preperation of Fs"),
            Events::OkRemoteInstall         => (false, "Remote Install"),
            Events::OkRemoteInstallCleanup  => (false, "Remote Cleanup"),
            // installer/Ping.rs
            Events::RunPing                 => (true, "Ping"),
            Events::RunPingSsh              => (true, "SSH Ping"),

            Events::OkPing                  => (false, "Ping"),
            Events::OkPingSsh               => (false, "SSH Ping"),
            // installer/Deploy.rs
            Events::RunNixBuild             => (true, "Nix Build"),
            Events::RunNixCopy              => (true, "Nix Copy"),
            Events::RunCreateProfile        => (true, "Create Profile"),
            Events::RunPrepSys              => (true, "Preperation of Sys"),
            Events::RunActivateSys          => (true, "Activate Sys"),
            Events::RunActivateBootloader   => (true, "Activate Bootloader"),
            Events::RunRebootSys            => (true, "Reboot"),

            Events::OkNixBuild              => (false, "Nix Build"),
            Events::OkNixCopy               => (false, "Nix Copy"),
            Events::OkCreateProfile         => (false, "Create Profile"),
            Events::OkPrepSys               => (false, "Preperation of Sys"),
            Events::OkActivateSys           => (false, "Activate Sys"),
            Events::OkActivateBootloader    => (false, "Activate Bootloader"),
            Events::OkRebootSys             => (false, "Reboot"),
            // installer/Drives.rs
            Events::RunPartEfi              => (true, "Partitionate Efi"),
            Events::RunPartRoot             => (true, "Partitionate Root"),
            Events::RunFormatEfi            => (true, "Format Efi"),
            Events::RunFormatRoot           => (true, "Format Root"),
            Events::RunCreateBootDir        => (true, "Create Boot Dir"),
            Events::RunMountBoot            => (true, "Mount Boot"),
            Events::RunMountRoot            => (true, "Mount Root"),

            Events::OkPartEfi               => (false, "Partitionate Efi"),
            Events::OkPartRoot              => (false, "Partitionate Root"),
            Events::OkFormatEfi             => (false, "Format Efi"),
            Events::OkFormatRoot            => (false, "Format Root"),
            Events::OkCreateBootDir         => (false, "Create Boot Dir"),
            Events::OkMountBoot             => (false, "Mount Boot"),
            Events::OkMountRoot             => (false, "Mount Root"),
            // installer/Helper.rs
            Events::RunGetHardware          => (true, "Get Hardware"),
            Events::RunGetDrives            => (true, "Get Drives"),

            Events::OkGetHardware           => (false, "Get Hardware"),
            Events::OkGetDrives             => (false, "Get Drives"),
            // installer/Inject.rs
            Events::RunInjectTailscale      => (true, "Inject Tailscale"),
            Events::RunInjectWifi           => (true, "Inject Wifi"),

            Events::OkInjectTailscale       => (false, "Inject Tailscale"),
            Events::OkInjectWifi            => (false, "Inject Wifi"),
        };

        let prefix = match run {
            true => "[ RUN ] ",
            false => "[ OK ] ",
        };

        println!("{}", (format!("{}{}", prefix, name)));
    }
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
