//! UI for managing the network by the user.

use bevy_egui::{EguiContexts, egui};
use naia_bevy_client::Client;
use playground_shared::bevy_ecs::{system::Local, prelude::EventWriter};
use super::events::TryConnectEvent;

#[derive(Default)]
pub(super) struct WindowState {
    ip: String,
    port: u16,
}

/// Displays a window for managing connections to servers.
pub(super) fn network_window_system(
    mut context: EguiContexts,
    mut client: Client,
    mut try_events: EventWriter<TryConnectEvent>,
    mut state: Local<WindowState>,
) {
    egui::Window::new("Network settings")
    .show(context.ctx_mut(), |ui| {
        if client.is_connected() { // Already connected
            ui.label(format!("Connected to {}", client.server_address().expect("Should have been able to access the server address")));
            if ui.button("Disconnect").clicked() {
                client.disconnect();
            }
        } else if client.is_connecting() { // Trying to connect
            ui.label("Connecting...");
        } else { // Not connected
            ui.text_edit_singleline(&mut state.ip);
            ui.add(egui::DragValue::new(&mut state.port));
            if ui.button("Connect").clicked() {
                // Send TryConnectEvent
                let event = TryConnectEvent::from_string(&state.ip, state.port, false);
                if let Ok(event) = event { try_events.send(event); }
            }
        }
    });
}