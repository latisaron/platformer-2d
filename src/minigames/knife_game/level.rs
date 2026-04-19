use bevy::{prelude::*};
use crate::minigames::shared::level::{Level};

fn knife_hash(level: usize) -> Option<usize> {
    match level {
        1 => Some(3),
        2 => Some(5),
        3 => Some(7),
        _ => None,
    }
}

pub fn setup_minigame_level(
    mut commands: Commands,
) {
    if let Some(target_score) = knife_hash(1) {
        commands.spawn((
        Level {
            current_value: 1,
            maximum_value: 3,
            target_score: target_score,
        },
    ));
    }
}