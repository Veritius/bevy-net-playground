use naia_bevy_shared::{Message, ProtocolPlugin};
use crate::version::version_string;

pub struct AuthenticationNetPlugin;

impl ProtocolPlugin for AuthenticationNetPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol.add_message::<AuthMessage>();
    }
}

/// Sent by the client to authenticate with the server.
#[derive(Message)]
pub struct AuthMessage {
    /// The version of the client trying to join.
    /// This should be a valid [semantic versioning](https://semver.org/) version, or the client will always be rejected by the server.
    /// The best way to get this string is by using [version_string].
    pub version: String,
}

impl AuthMessage {
    pub fn new() -> AuthMessage {
        Self {
            version: version_string(),
        }
    }
}