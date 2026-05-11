use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::minigames::{MinigameState, main::{current_level_state::{GunLevel, KnifeLevel, QuizLevel}, screen_manager::MainGameState}};

const INTERACT_RADIUS: f32 = 500.0;

#[derive(Component)]
pub enum Interactable {
    Knife,
    Shoot,
    Quiz,
    Gift,
}

pub fn check_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    player_query: Query<&Transform, With<Velocity>>,
    interactable_query: Query<(&Transform, &Interactable)>,
    mut main_game_next_state: ResMut<NextState<MainGameState>>,
    knife_level: Single<&KnifeLevel>,
    gun_level: Single<&GunLevel>,
    quiz_level: Single<&QuizLevel>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_transform) = player_query.single() else { 
        return 
    };
    let player_pos = player_transform.translation.truncate();

    let new_main_game_state =
        if knife_level.val >= 3 && gun_level.val >= 3 && quiz_level.val >= 3 {
            MainGameState::GiftScreen
        } else {
            MainGameState::NaughtyScreen
        };


    for (obj_transform, interactable) in &interactable_query {
        let obj_pos = obj_transform.translation.truncate();
        let dist = player_pos.distance(obj_pos);
        if dist <= INTERACT_RADIUS {
                        match interactable {
                Interactable::Knife => next_state.set(MinigameState::Knife),
                Interactable::Shoot => next_state.set(MinigameState::Shoot),
                Interactable::Quiz  => next_state.set(MinigameState::Quiz),
                Interactable::Gift => main_game_next_state.set(new_main_game_state),
            }
            return;
        }
    }
}