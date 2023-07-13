//! Changes the Naia replication status of an entity based on a component.

use naia_bevy_server::{CommandsExt, Server};
use playground_shared::bevy::prelude::*;
use playground_shared::bevy_ecs;

/// Entities with this component are marked to be replicated by Naia.
#[derive(Component)]
pub struct ReplicatedEntity;

pub(super) fn replication_component_system(
    mut server: Server,
    mut commands: Commands,
    added: Query<Entity, Added<ReplicatedEntity>>,
    mut removed: RemovedComponents<ReplicatedEntity>,
) {
    for entity in added.iter() {
        commands.entity(entity).enable_replication(&mut server);
    }

    for entity in removed.iter() {
        commands.entity(entity).disable_replication(&mut server);
    }
}