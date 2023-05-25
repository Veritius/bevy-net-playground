use naia_bevy_server::ReceiveEvents;
use playground_shared::bevy::prelude::*;

pub mod events;
pub mod room;
pub mod entity;

mod bind;

use bind::bind_to_ip;
use room::initialise_hub_room;

/// Manages the server and its events.
pub struct NetworkManagementPlugin;
impl Plugin for NetworkManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(bind_to_ip);
        app.add_startup_system(initialise_hub_room);
        
        app.add_systems((
            events::auth_events,
            events::connect_events,
            events::disconnect_events,
        ).chain().in_set(ReceiveEvents));
    }
}