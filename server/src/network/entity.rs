//! Things for associating a user ID with an entity, for a more ECS-friendly approach.

use std::collections::HashMap;
use naia_bevy_server::{UserKey, Server};
use playground_shared::bevy::prelude::*;
use playground_shared::bevy_ecs;

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

/// Used for removal detection by [disconnect_on_removed_component_system].
#[derive(Default)]
pub(super) struct RemovalDetectionMemory(HashMap<Entity, UserKey>);

/// This system disconnects users if the relevant component ([NetworkUserEntity]) is removed, using Bevy removal detection.
// this is a hell of a function name
pub(super) fn disconnect_on_removed_component_system(
    mut server: Server,
    mut memory: Local<RemovalDetectionMemory>,
    mut removals: RemovedComponents<NetworkUserEntity>,
    additions: Query<(Entity, &NetworkUserEntity), Added<NetworkUserEntity>>,
) {
    for (entity, key_comp) in additions.iter() {
        memory.0.insert(entity, key_comp.key);
    }

    for removed in removals.iter() {
        if !memory.0.contains_key(&removed) { continue; }
        // TODO: Figure out how to satisfy disconnect's arguments
        // server.user_mut(memory.0.get(&removed).unwrap()).disconnect(world);
        memory.0.remove(&removed);
    }
}