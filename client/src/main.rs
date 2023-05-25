pub mod network;

use bevy_egui::EguiPlugin;
use naia_bevy_client::{Plugin as ClientPlugin, ClientConfig};
use network::NetworkManagementPlugin;
use playground_shared::{bevy::prelude::*, network::protocol};

fn main() {
    let mut app = App::default();

    // Add default plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);

    // Set up naia client
    app.add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()));
    app.add_plugin(NetworkManagementPlugin);

    app.run();
}