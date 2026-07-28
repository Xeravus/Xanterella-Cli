use crate::prelude::*;

pub enum EventsFailed {
    /// Generall Errors
    Failed(String),
    FailedCmd(String),
    Tailscale(String),
    SerdeJson(String),
    Fs(String),
    Lsblk(String),
    ReadSymLink,

    /// Utils
    /// utils/Git.rs
    GitCommit,
    GitCheckout,
    GitCheckoutCreate,
    GitMerge,
    GitPr,

    /// utils/Check.rs
    CheckNix,

    /// utils/Config.rs
    ConfigCreateDir,
    ConfigGenBasic,

    /// Installer
    /// installer/Core.rs
    RemoteIntegration,
    RemotePrepFs,
    RemoteInstall,
    RemoteCleanup,

    /// installer/Ping.rs
    Ping,
    PingSsh,

    /// installer/Deploy.rs
    NixBuild,
    NixCopy,
    CreateProfile,
    PrepSys,
    ActivateSys,
    ActivateBootloader,
    RebootSys,

    /// installer/Drives.rs
    PartEfi,
    PartRoot,
    FormatEfi,
    FormatRoot,
    CreateBootDir,
    MountBoot,
    MountRoot,

    /// installer/Helper.rs
    GetHardware,
    GetDrives,

    /// installer/Inject.rs
    InjectTailscale,
    InjectWifi,
}

pub enum Events {
    /// Utils
    /// utils/Git.rs
    RunGitCommit,
    RunGitCheckout,
    RunGitCheckoutCreate,
    RunGitMerge,
    RunGitPr,

    OkGitCommit,
    OkGitCheckout,
    OkGitCheckoutCreate,
    OkGitMerge,
    OkGitPr,

    /// utils/Check.rs
    RunCheckNix,

    OkCheckNix,

    /// utils/Config.rs
    RunConfigCreateDir,
    RunConfigGenBasic,

    OkConfigCreateDir,
    OkConfigGenBasic,

    /// Installer
    /// installer/Core.rs
    RunRemoteIntegration,
    RunRemotePrepFs,
    RunRemoteInstall,
    RunRemoteInstallCleanup,

    OkRemoteIntegration,
    OkRemotePrepFs,
    OkRemoteInstall,
    OkRemoteInstallCleanup,

    /// installer/Ping.rs
    RunPing,
    RunPingSsh,

    OkPing,
    OkPingSsh,

    /// installer/Deploy.rs
    RunNixBuild,
    RunNixCopy,
    RunCreateProfile,
    RunPrepSys,
    RunActivateSys,
    RunActivateBootloader,
    RunRebootSys,

    OkNixBuild,
    OkNixCopy,
    OkCreateProfile,
    OkPrepSys,
    OkActivateSys,
    OkActivateBootloader,
    OkRebootSys,

    /// installer/Drives.rs
    RunPartEfi,
    RunPartRoot,
    RunFormatEfi,
    RunFormatRoot,
    RunCreateBootDir,
    RunMountBoot,
    RunMountRoot,

    OkPartEfi,
    OkPartRoot,
    OkFormatEfi,
    OkFormatRoot,
    OkCreateBootDir,
    OkMountBoot,
    OkMountRoot,

    /// installer/Helper.rs
    RunGetHardware,
    RunGetDrives,

    OkGetHardware,
    OkGetDrives,

    /// installer/Inject.rs
    RunInjectTailscale,
    RunInjectWifi,

    OkInjectTailscale,
    OkInjectWifi,
}
