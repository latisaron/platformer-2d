use bevy::{prelude::*, window::PrimaryWindow};
use crate::minigames::{MinigameState};

#[derive(States, Hash, PartialEq, Eq, Debug, Clone)]
pub enum MainGameState {
    StartScreen,
    NaughtyScreen,
    GiftScreen,
    PlayScreen,
}

#[derive(Component)]
pub struct ScreenManagerCleanup;

pub fn setup_start_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("main/start.png"),
            custom_size: Some(Vec2::new(window_width * 0.8, window_height * 0.8)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.),
        ScreenManagerCleanup,
    ));
}

pub fn setup_naughty_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("main/naughty.png"),
            custom_size: Some(Vec2::new(window_width * 0.8, window_height * 0.8)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.),
        ScreenManagerCleanup,
    ));
}

pub fn setup_gift_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("main/gift.png"),
            custom_size: Some(Vec2::new(window_width * 0.8, window_height * 0.8)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.),
        ScreenManagerCleanup,
    ));
}

pub fn exit_screen(
    mut main_game_state: ResMut<NextState<MainGameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        main_game_state.set(MainGameState::PlayScreen);
    }
}

pub fn cleanup_screens(
    mut commands: Commands,
    query: Query<Entity, With<ScreenManagerCleanup>>
) {
    for item in query {
        commands.entity(item).despawn();
    }
}