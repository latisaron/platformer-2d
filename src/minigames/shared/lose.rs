use bevy::{prelude::*};

use crate::minigames::shared::menu::{menu_action::MenuAction, state_management::GameState};

pub fn lose_level(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_action: ResMut<NextState<MenuAction>>
) {
    game_state.set(GameState::Menu);
    menu_action.set(MenuAction::PreLose);
}