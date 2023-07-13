use bevy::prelude::*;
use naia_bevy_shared::{Message, ProtocolPlugin};

/// Deals with player movement inputs.
pub(super) struct PlayerMovementPlugin;

impl ProtocolPlugin for PlayerMovementPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol.add_message::<PlayerMovement>();
    }
}

/// The player has moved somehow.
#[derive(Message)]
pub struct PlayerMovement {
    pub intent: Vec2
}