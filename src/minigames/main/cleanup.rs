use bevy::{prelude::*};

#[derive(Component)]
pub struct MainCleanup;

pub fn cleanup_main_game(
    mut commands: Commands,
    entities_query: Query<Entity, With<MainCleanup>>,
) {
    for entity in entities_query {
        commands.entity(entity).try_despawn();
    }
}
