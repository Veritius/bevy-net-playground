use std::{net::{IpAddr, AddrParseError, SocketAddr}, str::FromStr};
use naia_bevy_client::{Client, transport::udp::Socket, events::{ConnectEvent, RejectEvent, DisconnectEvent}};
use playground_shared::{bevy_ecs::prelude::EventReader, network::auth::AuthMessage, bevy::prelude::info};

pub(super) fn connect_events(
    client: Client,
    mut events: EventReader<ConnectEvent>,
) {
    // ConnectEvents don't carry any information, all we want is to know one exists
    if events.is_empty() { return; }
    events.clear();

    let server_address = client.server_address().unwrap();

    info!("Successfully connected to {}:{}", server_address.ip(), server_address.port());
}

pub(super) fn disconnect_events(
    mut events: EventReader<DisconnectEvent>,
) {
    if events.is_empty() { return; }
    events.clear();
    
    info!("Disconnected from remote server");
}

pub(super) fn reject_events(
    mut events: EventReader<RejectEvent>,
) {
    if events.is_empty() { return; }
    events.clear();

    info!("Remote server rejected connection");
}

pub(super) fn try_connect_system(
    mut events: EventReader<TryConnectEvent>,
    mut client: Client,
) {
    if events.is_empty() { return; }
    let event = events.iter().last().unwrap();

    // Deal with existing connection, if any
    match (event.close_existing, (client.is_connected() || client.is_connecting())) {
        (true, true) => { client.disconnect(); },
        (false, true) => { return; },
        _ => {},
    }

    // Authenticate and connect to server
    client.auth(AuthMessage::new());
    let socket_addr = SocketAddr::new(event.ip_address, event.port);
    let socket = Socket::new(&socket_addr, None);
    client.connect(socket);

    info!("Trying to connect to {}:{}", event.ip_address, event.port);
}

/// Write this event to try and connect to a server.
/// Last written event takes priority.
pub struct TryConnectEvent {
    close_existing: bool,
    ip_address: IpAddr,
    port: u16,
}

impl TryConnectEvent {
    /// Tries to parse the string as an IP address and returns a `TryConnectEvent`
    pub fn from_string(ip: &str, port: u16, close_existing: bool) -> Result<TryConnectEvent, AddrParseError> {
        let ip = IpAddr::from_str(ip);
        if ip.is_err() { return Err(ip.unwrap_err()); }

        Ok(Self {
            close_existing,
            ip_address: ip.unwrap(),
            port
        })
    }

    /// Creates the event from an address and port
    pub fn from_ip(ip: IpAddr, port: u16, close_existing: bool) -> TryConnectEvent {
        Self {
            close_existing,
            ip_address: ip,
            port
        }
    }
}

