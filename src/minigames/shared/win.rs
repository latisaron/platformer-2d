use bevy::{prelude::*};

use crate::minigames::shared::menu::{state_management::GameState};

pub fn win_level(
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Menu);
}