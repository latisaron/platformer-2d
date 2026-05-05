use bevy::{prelude::*, window::PrimaryWindow};
use crate::minigames::{
    MinigameState, main::current_level_state::GunLevel, shared::{level::Level, menu::{
        menu_action::MenuAction,
        menu_item_type::MenuItemType,
        state_management::{
            GameState,
            setup_menu,
        },
    }, score::{Score, reset_score}}, shooting_game::{LossState, gun::{GunCleanup, reset_gun}, level::{bullet_hash, target_score_hash, target_time_hash}, target::{TargetCleanup, reset_targets}, timer::{TimerCleanup, reset_timer}}
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
    loss_state: Res<State<LossState>>,
) {
    menu_action_state.set(MenuAction::None);
    let text =
        if *loss_state.get() == LossState::Timer {
            String::from("You ran out of time. Booohooo.")
        } else {
            String::from("You got too trigger-happy. No more bullets for you.")
        };
    setup_menu(
        commands,
        window,
        materials,
        meshes,
        vec![
            MenuItemType::Restart(String::from("Restart")),
            MenuItemType::Exit(String::from("Exit")),
        ],
        text,
        2);
}

pub fn setup_win_menu(
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    window: Single<& Window>,
    mut level: Single<&mut Level>,
    mut gun_level: Single<&mut GunLevel>,
) {
    menu_action_state.set(MenuAction::None);
    level.target_score = target_score_hash(level.current_value + 1);
    level.target_time = Some(target_time_hash(level.current_value + 1));
    level.bullets = Some(bullet_hash(level.current_value + 1));

    level.current_value += 1;
    gun_level.val += 1;
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
    // shared restart actions
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    // score
    score: Single<&mut Score>,
    //gun
    window: Query<&Window, With<PrimaryWindow>>,
    cleanup_gun_entities: Query<(Entity, &GunCleanup)>,
    // target
    cleanup_target_entities: Query<(Entity, &TargetCleanup)>,
    // timer
    level: Single<&Level>,
    cleanup_timer_entities: Query<(Entity, &TimerCleanup)>,

) {
    reset_gun(&mut commands, &window, &asset_server, &mut texture_atlas_layouts, &level, &cleanup_gun_entities);
    reset_timer(&level, &mut commands, &cleanup_timer_entities);
    reset_targets(&mut commands, &cleanup_target_entities);
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