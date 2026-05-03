use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::inventory::{PLAYER_MODEL_Z_INDEX, WHITE_FRAME_Z_INDEX};

const IMAGE_WIDTH_PERCENTAGE: f32 = 0.3;
const IMAGE_HEIGH_PERCENTAGE: f32 = 0.8;
const IMAGE_X_PERCENTAGE: f32 = 0.3;
const IMAGE_Y_PERCENTAGE: f32 = 0.3;

#[derive(Component)]
pub struct PlayerModel {
    pub hat: Option<String>,
    pub undershirt: Option<String>,
    pub outershirt: Option<String>,
    pub pants: Option<String>,
    pub shoes: Option<String>,
}

#[derive(Component)]
pub struct WhiteBackground;

pub fn setup_player_model(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/white_frame.png"),
            custom_size: Some(Vec2::new((IMAGE_WIDTH_PERCENTAGE + 0.05) * window_width, (IMAGE_HEIGH_PERCENTAGE + 0.05) * window_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 50., WHITE_FRAME_Z_INDEX),
        PlayerModel { hat: None, undershirt: None, outershirt: None, pants: None, shoes: None, },
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/raw_man.png"),
            custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, IMAGE_HEIGH_PERCENTAGE * window_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 50., PLAYER_MODEL_Z_INDEX),
        WhiteBackground,
    ));
}