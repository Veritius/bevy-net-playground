use playground_shared::bevy::prelude::*;
use playground_shared::bevy_ecs;

/// Marker component for replicating the [Transform] component.
/// If this component is removed, the [TransformDereplicated] message is sent, and the client will deal with it.
#[derive(Component)]
pub struct ReplicateTransform;

fn transform_replication_system(
    replicated: Query<Transform, With<ReplicateTransform>>,
) {
    
}