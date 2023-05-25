use naia_bevy_server::{RoomKey, Server};
use playground_shared::bevy::prelude::*;
use playground_shared::bevy_ecs;

#[derive(Resource)]
pub struct HubRoom(pub RoomKey);

/// Adds the [HubRoom] resource. This is a setup system.
pub(super) fn initialise_hub_room(
    mut commands: Commands,
    mut server: Server,
) {
    commands.insert_resource(HubRoom(server.make_room().key()));
}