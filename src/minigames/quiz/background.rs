use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, cleanup::CleanupQuiz};

pub fn create_background(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/generic_overlay.png"),
            custom_size: Some(Vec2::new(window_width, window_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        CleanupQuiz,
    ));
}