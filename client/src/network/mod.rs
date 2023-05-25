pub mod events;

mod ui;

use naia_bevy_client::ReceiveEvents;
use playground_shared::bevy::prelude::*;
use events::{try_connect_system, TryConnectEvent};
use ui::network_window_system;

/// Manages the client and its connection to the server.
pub struct NetworkManagementPlugin;
impl Plugin for NetworkManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryConnectEvent>();

        app.add_systems((
            events::connect_events,
            events::disconnect_events,
            events::reject_events,
        ).chain().in_set(ReceiveEvents));

        app.add_system(try_connect_system);
        app.add_system(network_window_system);
    }
}