pub mod network;

use std::time::Duration;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig};
use network::NetworkManagementPlugin;
use playground_shared::{bevy::{prelude::*, app::ScheduleRunnerSettings, diagnostic::DiagnosticsPlugin, log::LogPlugin}, network::protocol};

/// The amount of ticks per second the server will try to match.
const SERVER_TICK_RATE: u32 = 60;

fn main() {
    let mut app = App::default();

    // Set up bevy plugins
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(1.0 / SERVER_TICK_RATE as f64)));
    app.add_plugins(MinimalPlugins);
    app.add_plugin(LogPlugin::default());
    app.add_plugin(TransformPlugin);
    app.add_plugin(HierarchyPlugin);
    app.add_plugin(DiagnosticsPlugin);
    app.add_plugin(AssetPlugin::default());

    // Set up naia server
    app.add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()));
    app.add_plugin(NetworkManagementPlugin);

    app.run();
}