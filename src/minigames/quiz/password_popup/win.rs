use bevy::{prelude::*};

use crate::minigames::quiz::{QuizGameState, cleanup::CleanupQuiz, password_popup::{cleanup::CleanupPasswordPopup, popup::{POPUP_HEIGHT, POPUP_WIDTH, POPUP_X, POPUP_Y}}};

const DEFAULT_WIN_TIME: f32 = 5.;

#[derive(Component)]
pub struct WinFlashPopup {
    pub remaining_time: f32,
}

pub fn setup_win_flash_popup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
        commands.spawn((
        Sprite {
            image: asset_server.load("quiz_game/win.png"),
            custom_size: Some(Vec2::new(POPUP_WIDTH, POPUP_HEIGHT)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(POPUP_X, POPUP_Y, 8.),
        WinFlashPopup { remaining_time: DEFAULT_WIN_TIME },
        CleanupPasswordPopup,
        CleanupQuiz,
    ));
}

pub fn despawn_win_if_done(
    mut commands: Commands,
    time: Res<Time>,
    mut error_popup: Single<(&mut WinFlashPopup, Entity)>,
    mut next_state: ResMut<NextState<QuizGameState>>
) {
    let passed_time = time.delta_secs();

    if error_popup.0.remaining_time - passed_time <= 0. {
        commands.entity(error_popup.1).despawn();
        next_state.set(QuizGameState::Choosing);
    } else {
        error_popup.0.remaining_time -= passed_time;
    }
}
