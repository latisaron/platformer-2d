use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, cleanup::CleanupQuiz};

const IMAGE_WIDTH_PERCENTAGE: f32 = 0.15;
const IMAGE_HEIGH_PERCENTAGE: f32 = 0.1;

const IMAGE_Z_INDEX: f32 = 0.;

const PADDING: f32 = 20.;

#[derive(Component)]
pub struct RequestReviewButton;

pub fn create_request_button(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    window: &Single<&Window, With<PrimaryWindow>>,
) {
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/request_review_button.png"),
            custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, IMAGE_HEIGH_PERCENTAGE * window_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., -(window_height / 2. - IMAGE_HEIGH_PERCENTAGE * window_height / 2. - PADDING), IMAGE_Z_INDEX),
        RequestReviewButton,
        CleanupQuiz,
    ));
}

pub fn setup_review_request_button(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    create_request_button(&mut commands, &mut asset_server, &window);
}

pub fn handle_request_review_button_interaction(
    keys: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    button_query: Single<&Transform, With<RequestReviewButton>>,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    let window_width = window.width();
    let window_height = window.height();
    if keys.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);

            let button_lwr_x = button_query.translation.x - window_width * IMAGE_WIDTH_PERCENTAGE / 2.;
            let button_upr_x = button_query.translation.x + window_width * IMAGE_WIDTH_PERCENTAGE / 2.;
            let button_lwr_y = button_query.translation.y - window_height * IMAGE_HEIGH_PERCENTAGE / 2.;
            let button_upr_y = button_query.translation.y + window_height * IMAGE_HEIGH_PERCENTAGE / 2.;

            if x >= button_lwr_x && x <= button_upr_x && y >= button_lwr_y && y <= button_upr_y { 
                quiz_game_state.set(QuizGameState::PasswordPopup);
            }
        }
    }
}
