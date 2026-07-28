use crate::prelude::*;

pub enum EventsFailed {
    Failed(String),
    FailedCmd(String),

    Ping,
    PingSsh,

    CheckNix,
    
    GetHardware,
    GetDrives,

    GitCommit,
    GitCheckout,
    GitMerge,
    GitPr,
    GitDiff,

    PartEfi,
    PartRoot,
    FormatEfi,
    FormatRoot,
    CreateBootDir,
    MountBoot,
    MountRoot,

    InjectWifi,
    InjectTailscale,

    CreateDir,

    NixBuild,
    NixCopy,
    
    Fs,
    Lsblk,
    Tailscale,
    SerdeJson,
    Reboot,
    ActivateBootloader,
    ActivateSys,
    PrepSys,
    CreateProfile,
    ReadSymLink,
    RemotePrepFs,
}

pub enum Events {
    RunPing,
    OkPing,

    RunPingSsh,
    OkPingSsh,

    RunCheckNix,
    OkCheckNix, 

    RunGetHardware,
    OkGetHardware,
    //
    RunGitCommit,
    OkGitCommit,

    RunGitCheckout,
    OkGitCheckout,

    RunGitMerge,
    OkGitMerge,

    RunGitCheckoutCreate,
    OkGitCheckoutCreate,

    RunGitPr,
    OkGitPr,

    RunGitDiff,
    OkGitDiff,
    //
    RunRemoteIntegration,
    OkRemoteIntegration,

    RunRemoteInstall,
    OkRemoteInstall,
    //
    RunPartEfi,
    OkPartEfi,

    RunPartRoot,
    OkPartRoot,

    RunFormatEfi,
    OkFormatEfi,

    RunFormatRoot,
    OkFormatRoot,

    RunCreateBootDir,
    OkCreateBootDir,

    RunMountBoot,
    OkMountBoot,

    RunMountRoot,
    OkMountRoot,
    //
    RunInjectTailscale,
    OkInjectTailscale,

    RunInjectWifi,
    OkInjectWifi,
    //
    RunGetDrives,
    OkGetDrives,
    //
    RunConfigGenBasic,
    OkConfigGenBasic,

    RunConfigCreateDir,
    OkConfigCreateDir,

    RunReboot,
    OkReboot,

    RunActivateBootloader,
    OkActivateBootloader,

    RunActivateSys,
    OkActivateSys,

    RunPrepSys,
    OkPrepSys,

    RunCreateProfile,
    OkCreateProfile,

    RunNixCopy,
    OkNixCopy,

    RunNixBuild,
    OkNixBuild,

    RunRemoteInstallCleanup,
    OkRemoteInstallCleanup,

    RunRemotePrepFs,
    OkRemotePrepFs,
}
