use bevy::{prelude::*};

#[derive(Component)]
pub struct CleanupQuiz;

pub fn cleanup_quiz_game(
    mut commands: Commands,
    entities_query: Query<Entity, With<CleanupQuiz>>,
) {
    for entity in entities_query {
        commands.entity(entity).despawn();
    }
}
