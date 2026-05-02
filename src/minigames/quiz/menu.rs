use bevy::{prelude::*, window::PrimaryWindow};
use crate::minigames::{
    MinigameState, quiz::QuizGameState, shared::{level::Level, menu::{
        menu_action::MenuAction,
        menu_item_type::MenuItemType,
        state_management::{
            GameState,
            setup_menu,
        },
    }, score::{Score, reset_score}}, shooting_game::{LossState, gun::{GunCleanup, reset_gun}, level::{bullet_hash, target_score_hash, target_time_hash}, target::{TargetCleanup, reset_targets}, timer::{TimerCleanup, reset_timer}}
};

pub fn setup_quiz_menu(
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
        3);
}

pub fn continue_quiz_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    menu_action_state.set(MenuAction::None);
    game_state.set(GameState::Play);
}

pub fn exit_quiz_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut minigame_state: ResMut<NextState<MinigameState>>,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    menu_action_state.set(MenuAction::None);
    minigame_state.set(MinigameState::Main);
    game_state.set(GameState::Play);
    quiz_game_state.set(QuizGameState::None);
    
}