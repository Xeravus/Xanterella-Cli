pub mod config;
pub mod xanterella;
pub mod get;
pub mod git;
pub mod result;
pub mod install;
pub mod nix;

pub mod prelude;

pub use result::{Events, EventsFailed};
pub use git::{Branches, PrType, Git};
pub use config::{Data, Config};
pub use get::{User, Paths};
pub use xanterella::Xanterella;

pub use install::install::XanterellaInstall;
pub use crate::install::ping::Ping;
