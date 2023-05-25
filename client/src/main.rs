pub mod network;

use naia_bevy_client::{Plugin as ClientPlugin, ClientConfig};
use playground_shared::{bevy::prelude::*, network::protocol};

fn main() {
    let mut app = App::default();

    // Add default plugins
    app.add_plugins(DefaultPlugins);

    // Set up naia client
    app.add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()));

    app.run();
}