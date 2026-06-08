use bevy::{prelude::*};
use crate::minigames::{
    MinigameState, knife_game::{chopping_block::{CleanupChoppingBlock, reset_chopping_block}, knife::{ChoppingGameState, CleanupKnife, reset_knife}, level::knife_hash}, main::current_level_state::KnifeLevel, shared::{level::Level, menu::{
        menu_action::MenuAction,
        menu_item_type::MenuItemType,
        state_management::{
            GameState,
            setup_menu,
        },
    }, score::{Score, reset_score}}
};

pub fn setup_knife_menu(
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
    mut knife_level: Single<&mut KnifeLevel>,
) {
    menu_action_state.set(MenuAction::None);
    level.target_score = knife_hash(level.current_value + 1);
    knife_level.val += 1;
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
        String::from("You Won!"),
        2);
}

pub fn continue_knife_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut chopping_game_state: ResMut<NextState<ChoppingGameState>>
) {
    menu_action_state.set(MenuAction::None);
    chopping_game_state.set(ChoppingGameState::Playing);
    game_state.set(GameState::Play);
}

pub fn restart_knife_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut chopping_game_state: ResMut<NextState<ChoppingGameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
    level: Single<& Level>,
    // chopping block
    cleanup_chopping_block_entities: Query<(Entity, &CleanupChoppingBlock)>,
    // score
    score: Single<&mut Score>,
    // knife
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cleanup_knife_entities: Query<(Entity, &CleanupKnife)>,
) {
    reset_chopping_block(&mut commands, &mut asset_server, &mut materials, &mut meshes, &window, cleanup_chopping_block_entities, level);
    reset_knife(&mut commands, &mut materials, &mut meshes, &mut texture_atlas_layouts, &mut asset_server, &window, cleanup_knife_entities);
    reset_score(score);
    chopping_game_state.set(ChoppingGameState::Playing);
    menu_action_state.set(MenuAction::None);
    game_state.set(GameState::Play);
}

pub fn exit_knife_game(
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut minigame_state: ResMut<NextState<MinigameState>>,
    mut chopping_game_state: ResMut<NextState<ChoppingGameState>>,
) {
    menu_action_state.set(MenuAction::None);
    minigame_state.set(MinigameState::Main);
    game_state.set(GameState::Play);
    chopping_game_state.set(ChoppingGameState::Playing);
}