use bevy::{prelude::*};
use crate::minigames::{MinigameState};

pub fn choose_minigame(
    mut current_minigame: ResMut<NextState<MinigameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyK) {
        current_minigame.set(MinigameState::Knife);
    } else if keys.just_pressed(KeyCode::KeyL) {
        current_minigame.set(MinigameState::Shoot);
    }
     else if keys.just_pressed(KeyCode::KeyJ) {
        current_minigame.set(MinigameState::Quiz);
    }
}