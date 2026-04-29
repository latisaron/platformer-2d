use bevy::{prelude::*};
use crate::minigames::shared::level::{CleanupLevel, Level};

pub fn target_score_hash(level: usize) -> String {
    match level {
        1 => String::from("copacel-piscotel"),
        2 => String::from("sex-pula-pistol"),
        3 => String::from("dungeon-crawler-carl"),
        _ => String::from("238skammdasold9328321831l23m12412nx7hb12#@!$1msxix12$%!@#!@41k5214l12412312k31mX@!#"),
    }
}

pub fn setup_minigame_level(
    mut commands: Commands,
) {
    commands.spawn((
        Level {
            current_value: 1,
            target_score: 1,
            target_time: None,
            bullets: None,
            secret_password: Some(target_score_hash(1)),
        },
        CleanupLevel
    ));
}