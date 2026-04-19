use bevy::{prelude::*};
use crate::minigames::{
    MinigameState, knife_game::{chopping_block::{CleanupChoppingBlock, reset_chopping_block}, knife::{ChoppingGameState, CleanupKnife, reset_knife}}, shared::{menu::{
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
            MenuItemType::Continue,
            MenuItemType::Restart,
            MenuItemType::Exit,
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
            MenuItemType::Restart,
            MenuItemType::Exit,
        ],
        String::from("You Lost!"),
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
    // chopping block
    cleanup_chopping_block_entities: Query<(Entity, &CleanupChoppingBlock)>,
    // score
    score: Single<&mut Score>,
    // knife
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cleanup_knife_entities: Query<(Entity, &CleanupKnife)>,
) {
    reset_chopping_block(&mut commands, &mut asset_server, &mut materials, &mut meshes, &window, cleanup_chopping_block_entities);
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
) {
    menu_action_state.set(MenuAction::None);
    minigame_state.set(MinigameState::Main);
    game_state.set(GameState::Play);
}