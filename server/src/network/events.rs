use naia_bevy_server::{Server, events::{AuthEvents, ConnectEvent, DisconnectEvent}};
use playground_shared::{bevy::prelude::*, network::auth::AuthMessage, GAME_VERSION};
use crate::network::entity::NetworkUserEntity;

use super::room::HubRoom;

pub(super) fn auth_events(
    mut server: Server,
    mut events: EventReader<AuthEvents>,
) {
    for events in events.iter() {
        for (user_key, auth) in events.read::<AuthMessage>() {
            // Check client version
            let client_version = playground_shared::semver::Version::parse(&auth.version);
            if client_version.is_err() || client_version.unwrap() != *GAME_VERSION { server.reject_connection(&user_key); return; }

            // All checks succeeded
            server.accept_connection(&user_key);
        }
    }
}

pub(super) fn connect_events(
    mut commands: Commands,
    mut server: Server,
    hub_room: Res<HubRoom>,
    mut events: EventReader<ConnectEvent>,
) {
    for ConnectEvent(user_key) in events.iter() {
        let address = server
        .user_mut(user_key)
        .enter_room(&hub_room.0)
        .address();

        let user_entity = commands.spawn(NetworkUserEntity { key: user_key.clone() }).id();

        info!("Player joined with address {} and was assigned entity {:?}", address, user_entity);
    }
}

pub(super) fn disconnect_events(
    mut commands: Commands,
    mut events: EventReader<DisconnectEvent>,
    users: Query<(Entity, &NetworkUserEntity)>,
) {
    for DisconnectEvent(user_key, user) in events.iter() {
        let user_entity = users
            .iter()
            .filter(|x| x.1.key == *user_key)
            .nth(0)
            .expect("UserKey should have had an associated entity!")
            .0;

        commands.entity(user_entity).despawn_recursive();

        info!("Player with address {} and entity {:?} disconnected", user.address, user_entity);
    }
}