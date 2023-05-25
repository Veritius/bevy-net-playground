pub use bevy;
pub use naia_bevy_shared;
pub use semver;

pub mod network;

use bevy::render::once_cell::sync::Lazy;
use semver::Version;

/// The version of the game.
pub static GAME_VERSION: Lazy<Version> = Lazy::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());