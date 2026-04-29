use bevy::{prelude::*};

use crate::minigames::quiz::password_popup::cleanup::CleanupPasswordPopup;

pub const POPUP_WIDTH: f32 = 800.;
pub const POPUP_HEIGHT: f32 = 600.;

const POPUP_Z_INDEX: f32 = 5.;

#[derive(Component)]
pub struct PasswordPopup;

pub fn create_password_popup(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/popup.png"),
            custom_size: Some(Vec2::new(POPUP_WIDTH, POPUP_HEIGHT)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 0., POPUP_Z_INDEX),
        PasswordPopup,
        CleanupPasswordPopup,
    ));
}

pub fn setup_password_popup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    create_password_popup(&mut commands, &mut asset_server);
}