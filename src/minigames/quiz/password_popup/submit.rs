use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{
    password_popup::{
        cleanup::CleanupPasswordPopup,
        popup::{
            POPUP_HEIGHT,
            POPUP_WIDTH,
            PasswordPopup,
        },
        password::{Password},
    },
};
use crate::minigames::quiz::QuizGameState;

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
    mut asset_server: ResMut<AssetServer>,
) {
    create_submit_button(&mut commands, &mut asset_server);
}

pub fn handle_submit_click(
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<MouseButton>>,
    password: Single<&Password>,
    submit_button: Single<&Transform, With<SubmitButton>>,
    mut current_state: ResMut<NextState<QuizGameState>>,
) {
    if keys.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);

            let button_lwr_x = submit_button.translation.x - BUTTON_WIDTH / 2.;
            let button_upr_x = submit_button.translation.x + BUTTON_WIDTH / 2.;
            let button_lwr_y = submit_button.translation.y - BUTTON_HEIGHT / 2.;
            let button_upr_y = submit_button.translation.y + BUTTON_HEIGHT / 2.;

            if x >= button_lwr_x && x <= button_upr_x && y >= button_lwr_y && y <= button_upr_y {
                if password.correct() {
                    current_state.set(QuizGameState::PasswordPopupWin);
                } else {
                    current_state.set(QuizGameState::PasswordPopupError);
                }
            }
        }
    }
}