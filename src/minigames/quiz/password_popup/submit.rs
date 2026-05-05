use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::{main::current_level_state::QuizLevel, quiz::{cleanup::CleanupQuiz, level::target_score_hash, password_popup::{
        cleanup::CleanupPasswordPopup, password::Password, popup::{
            POPUP_HEIGHT,
            POPUP_WIDTH,
            PasswordPopup,
        }
    }}, shared::{level::Level, menu::menu_action::MenuAction, score::{Score, increase_score}}};
use crate::minigames::quiz::QuizGameState;

const BUTTON_WIDTH: f32 = 320.;
const BUTTON_HEIGHT: f32 = 80.;

const BUTTON_Z_INDEX: f32 = 16.;

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
        CleanupQuiz,
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
    submit_button: Single<&Transform, With<SubmitButton>>,
    mut password: Single<&mut Password>,
    mut current_state: ResMut<NextState<QuizGameState>>,
    mut menu_state: ResMut<NextState<MenuAction>>,
    mut level: Single<&mut Level>,
    mut score: Single<&mut Score>,
    mut quiz_level: Single<&QuizLevel>,
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
                    quiz_level.val +=1 ;
                    level.current_value += 1;
                    let new_secret = target_score_hash(level.current_value);
                    level.secret_password = Some(new_secret.clone());
                    password.change_secret(level.current_value);
                    password.current_password = String::from("");
                    increase_score(&mut score);
                    if score.0 == 3 {
                        menu_state.set(MenuAction::PreWin);
                    } 
                } else {
                    current_state.set(QuizGameState::PasswordPopupError);
                }
            }
        }
    }
}