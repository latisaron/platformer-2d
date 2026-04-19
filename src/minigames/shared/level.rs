use bevy::{prelude::*};

#[derive(Component)]
pub struct CleanupLevel;

#[derive(Component)]
pub struct Level {
    pub current_value: usize,
    pub maximum_value: usize,
    pub target_score: usize,
}

pub fn cleanup_level(
    mut commands: Commands,
    level_cleanups: Query<(Entity, &CleanupLevel)>
) {
    for level_cleanup in level_cleanups {
        commands.entity(level_cleanup.0).despawn();
    }
}