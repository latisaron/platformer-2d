use bevy::{prelude::*};
use crate::minigames::shared::level::{CleanupLevel, Level};

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
) {
    commands.spawn((
        Level {
            current_value: 1,
            target_score: knife_hash(1),
            target_time: None,
            bullets: None,
        },
        CleanupLevel
    ));
}