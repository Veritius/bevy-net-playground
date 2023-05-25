use std::{env, net::{SocketAddr, IpAddr}, str::FromStr};
use naia_bevy_server::{Server, transport::udp::Socket};
use playground_shared::bevy::prelude::info;

/// Binds to an IP and port defined in environment variables at startup.
pub(super) fn bind_to_ip(
    mut server: Server,
) {
    let ip = IpAddr::from_str(&env::var("BIND_IP").expect("IP address was not set.")).expect("IP address was not valid");
    let port = str::parse::<u16>(&env::var("BIND_PORT").expect("Port was not set")).expect("Port was not valid");
    let socket_addr = SocketAddr::new(ip, port);

    let socket = Socket::new(&socket_addr, None);
    server.listen(socket);

    info!("Server bound to {}:{}", ip, port);
}