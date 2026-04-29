use bevy::{prelude::*};

use crate::minigames::quiz::password_popup::{cleanup::CleanupPasswordPopup, popup::{POPUP_HEIGHT, POPUP_WIDTH, PasswordPopup}};

const BUTTON_WIDTH: f32 = 320.;
const BUTTON_HEIGHT: f32 = 80.;

const BUTTON_Z_INDEX: f32 = 6.;

#[derive(Component)]
pub struct SubmitButton;

pub fn create_submit_button(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/submit_button.png"),
            custom_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(POPUP_WIDTH / 4., - POPUP_HEIGHT * 3. / 8., BUTTON_Z_INDEX),
        SubmitButton,
        CleanupPasswordPopup,
    ));
}

pub fn setup_submit_button(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    create_submit_button(&mut commands, &mut asset_server);
}
