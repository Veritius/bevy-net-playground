use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig};
use playground_shared::{bevy::prelude::*, network::protocol};

fn main() {
    let mut app = App::default();

    // Add default plugins
    app.add_plugins(DefaultPlugins);

    // Set up naia server
    app.add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()));

    app.run();
}