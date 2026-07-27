use crate::xanterella::utils::git::*;
use crate::prelude::*;

pub enum EventsFailed {
    Failed(String),
    FailedCmd(String),

    Ping(String),
    PingSsh(String),

    CheckNix,
    
    GetHardware,

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

    Lsblk,
}

pub enum Events {
    RunPing(String),
    OkPing(String),

    RunPingSsh(String),
    OkPingSsh(String),

    RunCheckNix,
    OkCheckNix, 

    RunGetHardware(String),
    OkGetHardware(String),
    //
    RunGitCommit(String),
    OkGitCommit(String),

    RunGitCheckout(Branches),
    OkGitCheckout(Branches),

    RunGitMerge,
    OkGitMerge,

    RunGitCheckoutCreate(Branches),
    OkGitCheckoutCreate(Branches),

    RunGitPr(PrType),
    OkGitPr(PrType),

    RunGitDiff,
    OkGitDiff,
    //
    RunRemoteIntegration(String),
    OkRemoteIntegration(String),

    RunRemoteInstall(String),
    OkRemoteInstall(String),
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
}
