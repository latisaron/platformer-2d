use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, password::Password, password_popup::{cleanup::CleanupPasswordPopup, popup::{POPUP_HEIGHT, POPUP_WIDTH}}};

const BUTTON_WIDTH: f32 = 320.;
const BUTTON_HEIGHT: f32 = 80.;

const BUTTON_Z_INDEX: f32 = 6.;

#[derive(Component)]
pub struct CloseButton;

pub fn create_close_button(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/close_button.png"),
            custom_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(- POPUP_WIDTH / 4., - POPUP_HEIGHT * 3. / 8., BUTTON_Z_INDEX),
        CloseButton,
        CleanupPasswordPopup,
    ));
}

pub fn setup_close_button(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    create_close_button(&mut commands, &mut asset_server);
}

pub fn handle_close_click(
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<MouseButton>>,
    close_button: Single<&Transform, With<CloseButton>>,
    mut password: Single<&mut Password>,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    if keys.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);

            let button_lwr_x = close_button.translation.x - BUTTON_WIDTH / 2.;
            let button_upr_x = close_button.translation.x + BUTTON_WIDTH / 2.;
            let button_lwr_y = close_button.translation.y - BUTTON_HEIGHT / 2.;
            let button_upr_y = close_button.translation.y + BUTTON_HEIGHT / 2.;

            if x >= button_lwr_x && x <= button_upr_x && y >= button_lwr_y && y <= button_upr_y {
                quiz_game_state.set(QuizGameState::Choosing);
                password.current_password = String::from("");
            }
        }
    }
}