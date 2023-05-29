//! Things for associating a user ID with an entity, for a more ECS-friendly approach.

use std::collections::HashMap;
use naia_bevy_server::UserKey;
use naia_server::Server as NaiaServer;
use playground_shared::bevy::prelude::*;
use playground_shared::bevy_ecs;
use playground_shared::bevy_ecs::system::SystemState;
use playground_shared::naia_bevy_shared::WorldMut;

/// Used for removal detection.
#[derive(Resource, Default)]
pub(super) struct RemovalDetectionMemory(HashMap<Entity, UserKey>);

pub enum TryDisconnectEvent {
    Entity(Entity),
    UserKey(UserKey),
}

pub fn disconnect_exclusive_system(
    world: &mut World,
) {
    let mut state = SystemState::<(
        Query<Entity, &NetworkUserEntity>,
        EventReader<TryDisconnectEvent>,
        ResMut<RemovalDetectionMemory>,
        Commands,
    )>::new(world);
    let mut state_mut = state.get_mut(world);

    // Get all userkeys pending disconnection
    let mut keys_to_disconnect: Vec<UserKey> = Vec::with_capacity(state_mut.1.iter().len());
    for event in state_mut.1.iter() {
        match event {
            TryDisconnectEvent::Entity(entity) => {
                match state_mut.0.get(*entity) {
                    Ok(entity) => {
                        // Remove from memory
                        keys_to_disconnect.push(*state_mut.2.0.get(&entity)
                            .expect("Entity was not in removal memory"));
                        state_mut.2.0.remove(&entity);
                        state_mut.3.entity(entity).despawn();
                    },
                    Err(_) => {},
                }
            },
            TryDisconnectEvent::UserKey(user_key) => {
                keys_to_disconnect.push(*user_key);
                let mut ent = Entity::PLACEHOLDER;
                for (key, value) in state_mut.2.0.iter() {
                    if value != user_key { continue; }
                    ent = key.clone();
                    break;
                }

                // Delete from map and entities
                if ent != Entity::PLACEHOLDER {
                    state_mut.2.0.remove(&ent);
                    state_mut.3.entity(ent).despawn();
                }
            },
        }
    }

    // Disconnect all players
    world.resource_scope(|world, mut server: Mut<NaiaServer<Entity>>| {
        for key in keys_to_disconnect {
            let world_mut = WorldMut::new(world);
            server.user_mut(&key).disconnect(world_mut);
        }
    });
}

/// Associates network information with this entity.
/// Two entities should not have identical `UserKey`s.
/// 
/// Ideally, this is used to create an 'abstract' entity that isn't interacted with by the player, but is used to associate information with the user.
/// 
/// When the user disconnects, the attached entity is despawned. This behavior may change in future.
#[derive(Component)]
pub struct NetworkUserEntity {
    pub key: UserKey,
}

/// This system disconnects users if the relevant component ([NetworkUserEntity]) is removed, using Bevy removal detection.
// this is a hell of a function name
pub(super) fn disconnect_on_removed_component_system(
    mut memory: ResMut<RemovalDetectionMemory>,
    mut removals: RemovedComponents<NetworkUserEntity>,
    mut events: EventWriter<TryDisconnectEvent>,
    additions: Query<(Entity, &NetworkUserEntity), Added<NetworkUserEntity>>,
) {
    for (entity, key_comp) in additions.iter() {
        memory.0.insert(entity, key_comp.key);
    }

    for removed in removals.iter() {
        // Check this wasn't dealt with by the exclusive system
        if !memory.0.contains_key(&removed) { continue; }
        events.send(TryDisconnectEvent::Entity(removed));
    }
}