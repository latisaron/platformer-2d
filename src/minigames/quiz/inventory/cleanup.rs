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

#[derive(Component)]
pub struct CleanupCategory;

pub fn cleanup_category_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<CleanupCategory>>,
) {
    for entity in entities_query {
        commands.entity(entity).despawn();
    }
}