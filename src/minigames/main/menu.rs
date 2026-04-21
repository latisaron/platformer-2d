use std::process::exit;
use bevy::{prelude::*};

use crate::minigames::shared::menu::{menu_action::MenuAction, menu_item_type::MenuItemType, state_management::{GameState, setup_menu}};

pub fn exit_game() {
    exit(0);
}

pub fn setup_main_menu(
    commands: Commands,
    window: Single<& Window>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut menu_action_state: ResMut<NextState<MenuAction>>,
) {
    menu_action_state.set(MenuAction::None);
    setup_menu(
        commands,
        window,
        materials,
        meshes,
        vec![
            MenuItemType::Continue(String::from("Continue")),
            MenuItemType::Exit(String::from("Exit")),
        ],
        String::from("Menu"),
        2);
}

pub fn continue_main_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    menu_action_state.set(MenuAction::None);
    game_state.set(GameState::Play);
}