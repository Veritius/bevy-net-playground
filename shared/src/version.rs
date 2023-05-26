//! Things related to the game version used for denying access to players with the wrong version of the game.

use semver::Version;
use once_cell::sync::Lazy;

/// The current version of the game. This is based on the Cargo crate version.
pub static GAME_VERSION: Lazy<Version> = Lazy::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());

/// Returns a (probably) SemVer-compliant string based on [GAME_VERSION].
pub fn version_string() -> String {
    let mut string = format!("{}.{}.{}", GAME_VERSION.major, GAME_VERSION.minor, GAME_VERSION.patch);
    if !GAME_VERSION.pre.is_empty() { string.push_str(&format!("-{}", GAME_VERSION.pre)) }
    if !GAME_VERSION.build.is_empty() { string.push_str(&format!("+{}", GAME_VERSION.build)) }

    string
}

#[test]
fn check_game_version_is_valid() {
    if *GAME_VERSION != Version::parse(&version_string()).expect("Version string could not be parsed") {
        panic!("Parsed version string from version_string did not match GAME_VERSION");
    }
}