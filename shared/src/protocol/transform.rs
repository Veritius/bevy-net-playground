use bevy::prelude::Transform;
use naia_bevy_shared::{Message, ProtocolPlugin, Channel, ChannelDirection, ChannelMode, ReliableSettings};

/// Updates the player's Transform component.
pub(super) struct TransformUpdatePlugin;

impl ProtocolPlugin for TransformUpdatePlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol.add_channel::<TransformUpdateChannel>(
            ChannelDirection::ServerToClient,
            ChannelMode::UnorderedReliable(ReliableSettings::default()));

        protocol.add_message::<SetTransform>();
        protocol.add_message::<TransformDereplicated>();
    }
}

#[derive(Channel)]
pub struct TransformUpdateChannel;

#[derive(Message)]
pub struct SetTransform(pub Transform);

/// Sent if a [Transform] component stops being replicated.
/// Dealt with on the client side of things, on a case-by-case basis.
#[derive(Message)]
pub struct TransformDereplicated;