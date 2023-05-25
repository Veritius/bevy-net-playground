//! Things for associating a user ID with an entity, for a more ECS-friendly approach.

use naia_bevy_server::UserKey;
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