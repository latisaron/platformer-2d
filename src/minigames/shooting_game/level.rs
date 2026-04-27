use bevy::{prelude::*};
use crate::minigames::shared::level::{CleanupLevel, Level};

pub fn target_score_hash(level: usize) -> usize {
    match level {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => 133337,
    }
}

pub fn target_time_hash(level: usize) -> f32 {
    match level {
        1 => 20.,
        2 => 30.,
        3 => 45.,
        _ => 133337.,
    }
}

pub fn bullet_hash(level: usize) -> usize {
    match level {
        1 => 50,
        2 => 55,
        3 => 61,
        _ => 133333337,
    }
}

pub fn setup_minigame_level(
    mut commands: Commands,
) {
    commands.spawn((
        Level {
            current_value: 1,
            target_score: target_score_hash(1),
            target_time: Some(target_time_hash(1)),
            bullets: Some(bullet_hash(1)),
        },
        CleanupLevel
    ));
}