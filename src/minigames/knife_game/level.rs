use bevy::{prelude::*};
use crate::minigames::{main::current_level_state::KnifeLevel, shared::level::{CleanupLevel, Level}};

pub fn knife_hash(level: usize) -> usize {
    match level {
        1 => 3,
        2 => 5,
        3 => 7,
        _ => 1337,
    }
}

pub fn setup_minigame_level(
    mut commands: Commands,
    knife_level: Single<&KnifeLevel>,
) {
    let val = knife_level.val;
    commands.spawn((
        Level {
            current_value: val,
            target_score: knife_hash(val),
            target_time: None,
            bullets: None,
            secret_password: None,
        },
        CleanupLevel
    ));
}