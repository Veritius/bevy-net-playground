use naia_bevy_shared::{Message, ProtocolPlugin};

use crate::version_to_string;

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
    /// You should use `version_to_string` to set this.
    pub version: String,
}

impl AuthMessage {
    pub fn new() -> AuthMessage {
        Self {
            version: version_to_string(),
        }
    }
}