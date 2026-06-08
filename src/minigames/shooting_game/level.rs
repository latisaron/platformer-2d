use bevy::{prelude::*};
use crate::minigames::{main::current_level_state::{GunLevel, KnifeLevel}, shared::level::{CleanupLevel, Level}};

pub fn target_score_hash(level: usize) -> usize {
    match level {
        1 => 25,
        2 => 35,
        3 => 50,
        _ => 133337,
    }
}

pub fn target_time_hash(level: usize) -> f32 {
    match level {
        1 => 17.,
        2 => 28.,
        3 => 38.,
        _ => 133337.,
    }
}

pub fn bullet_hash(level: usize) -> usize {
    match level {
        1 => 40,
        2 => 43,
        3 => 58,
        _ => 133333337,
    }
}

pub fn setup_minigame_level(
    mut commands: Commands,
    gun_level: Single<&GunLevel>,
) {
    let val = gun_level.val;
    commands.spawn((
        Level {
            current_value: val,
            target_score: target_score_hash(val),
            target_time: Some(target_time_hash(val)),
            bullets: Some(bullet_hash(val)),
            secret_password: None,
        },
        CleanupLevel
    ));
}