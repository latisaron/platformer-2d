use bevy::{prelude::*};

use crate::minigames::quiz::QuizGameState;

#[derive(Component)]
pub struct CleanupQuiz;

pub fn cleanup_quiz_game(
    mut commands: Commands,
    entities_query: Query<Entity, With<CleanupQuiz>>,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    for entity in entities_query {
        commands.entity(entity).try_despawn();
    }
    quiz_game_state.set(QuizGameState::None);
}

pub fn reset_quiz_game_state(
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    quiz_game_state.set(QuizGameState::Choosing);
}