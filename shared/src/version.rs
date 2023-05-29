//! Things related to the game version used for denying access to players with the wrong version of the game.

use semver::Version;
use once_cell::sync::Lazy;

/// The current version of the game. This is based on the Cargo crate version.
pub static GAME_VERSION: Lazy<Version> = Lazy::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());

/// The current version of the game, as a SemVer-compliant string.
pub static GAME_VERSION_STRING: Lazy<String> = Lazy::new(|| format!("{}", *GAME_VERSION));

#[test]
fn check_game_version_is_valid() {
    if *GAME_VERSION != Version::parse(&*GAME_VERSION_STRING).expect("Version string could not be parsed") {
        panic!("Parsed version string from version_string did not match GAME_VERSION");
    }
}