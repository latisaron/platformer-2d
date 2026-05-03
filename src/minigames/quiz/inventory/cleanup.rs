use bevy::{prelude::*};

#[derive(Component)]
pub struct CleanupInventory;

pub fn cleanup_inventory_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<CleanupInventory>>,
) {
    for entity in entities_query {
        commands.entity(entity).despawn();
    }
}