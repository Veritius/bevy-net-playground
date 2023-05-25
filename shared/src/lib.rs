pub use bevy;
pub use naia_bevy_shared;
pub use once_cell;
pub use semver;
pub use bevy::ecs as bevy_ecs;

pub mod network;

use semver::Version;
use once_cell::sync::Lazy;

/// The version of the game.
pub static GAME_VERSION: Lazy<Version> = Lazy::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());

/// Returns a (probably) SemVer-compliant string based on [GAME_VERSION].
pub fn version_string() -> String {
    let mut string = format!("{}.{}.{}", GAME_VERSION.major, GAME_VERSION.minor, GAME_VERSION.patch);
    if !GAME_VERSION.pre.is_empty() { string.push_str(&format!("-{}", GAME_VERSION.pre)) }
    if !GAME_VERSION.build.is_empty() { string.push_str(&format!("+{}", GAME_VERSION.build)) }

    string
}