use bevy::prelude::*;
use naia_bevy_shared::{Message, ProtocolPlugin, Channel, ChannelDirection, ChannelMode, TickBufferSettings};

/// Deals with player input.
pub(super) struct PlayerInputPlugin;

impl ProtocolPlugin for PlayerInputPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol.add_channel::<PlayerInputChannel>(
            ChannelDirection::ClientToServer,
            ChannelMode::TickBuffered(TickBufferSettings::default()));
        protocol.add_message::<PlayerMovement>();
    }
}

/// Player input, such as movement.
#[derive(Channel)]
pub struct PlayerInputChannel;

/// Movement input by a player.
#[derive(Message)]
pub struct PlayerMovement {
    pub direction: Vec2
}