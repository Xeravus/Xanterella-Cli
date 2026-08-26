#[derive(Debug, Clone, PartialEq)]
pub enum EventsFailed {
    /// Generall Errors
    Failed(String),
    FailedCmd(String),
    Tailscale(String),
    SerdeJson(String),
    Fs(String),
    Lsblk(String),
    ReadSymLink(String),

    /// Utils
    /// utils/Git.rs
    GitCommit(String),
    GitCheckout(String),
    GitCheckoutCreate(String),
    GitMerge(String),
    GitPr(String),

    /// utils/Check.rs
    CheckNix(String),

    /// utils/Config.rs
    ConfigCreateDir(String),
    ConfigGenBasic(String),

    /// Installer
    /// installer/Ping.rs
    Ping(String),
    PingSsh(String),

    /// installer/Deploy.rs
    NixBuild(String),
    NixCopy(String),
    CreateProfile(String),
    PrepSys(String),
    ActivateSys(String),
    ActivateBootloader(String),
    RebootSys(String),

    /// installer/Drives.rs
    PartEfi(String),
    PartRoot(String),
    FormatEfi(String),
    FormatRoot(String),
    CreateBootDir(String),
    MountBoot(String),
    MountRoot(String),

    /// installer/Helper.rs
    GetHardware(String),
    GetDrives(String),

    /// installer/Inject.rs
    InjectTailscale(String),
    InjectWifi(String),

    /// Prolyxena
    Prolyxena(String),
}

#[derive(Debug, Clone, PartialEq)]
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
