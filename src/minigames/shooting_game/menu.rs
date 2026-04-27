use bevy::{prelude::*};
use crate::minigames::{
    MinigameState,
    shared::{level::Level, menu::{
        menu_action::MenuAction,
        menu_item_type::MenuItemType,
        state_management::{
            GameState,
            setup_menu,
        },
    }, score::{Score, reset_score}}, shooting_game::level::{bullet_hash, target_score_hash, target_time_hash}
};

pub fn setup_shoot_menu(
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
            MenuItemType::Restart(String::from("Restart")),
            MenuItemType::Exit(String::from("Exit")),
        ],
        String::from("Menu"),
        3);
}

pub fn setup_lose_menu(
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
            MenuItemType::Restart(String::from("Restart")),
            MenuItemType::Exit(String::from("Exit")),
        ],
        String::from("You Lost!"),
        2);
}

pub fn setup_win_menu(
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    window: Single<& Window>,
    mut level: Single<&mut Level>,
) {
    menu_action_state.set(MenuAction::None);
    level.target_score = target_score_hash(level.current_value + 1);
    level.target_time = Some(target_time_hash(level.current_value + 1));
    level.bullets = Some(bullet_hash(level.current_value + 1));

    level.current_value += 1;
    setup_menu(
        commands,
        window,
        materials,
        meshes,
        vec![
            MenuItemType::Restart(String::from("Continue")),
            MenuItemType::Exit(String::from("Exit")),
        ],
        String::from("You Little Shapshooter you. You WON!"),
        2);
}

pub fn continue_shoot_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    menu_action_state.set(MenuAction::None);
    game_state.set(GameState::Play);
}

pub fn restart_shoot_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
    // chopping block
    // score
    score: Single<&mut Score>,
    // knife
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    reset_score(score);
    menu_action_state.set(MenuAction::None);
    game_state.set(GameState::Play);
}

pub fn exit_shoot_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut minigame_state: ResMut<NextState<MinigameState>>,
) {
    menu_action_state.set(MenuAction::None);
    minigame_state.set(MinigameState::Main);
    game_state.set(GameState::Play);
}