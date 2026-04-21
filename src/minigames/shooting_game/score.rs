use bevy::{prelude::*};
use crate::minigames::shared::score::{setup_score};
use crate::minigames::shared::level::{Level};

pub fn setup_minigame_score(
    commands: Commands,
    query: Query<&Level>
) {
    if let Ok(level) = query.single() {
        setup_score(commands, level.target_score);
    }
}