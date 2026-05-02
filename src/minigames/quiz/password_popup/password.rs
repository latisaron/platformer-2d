use bevy::{prelude::*};

use crate::minigames::{quiz::password_popup::cleanup::CleanupPasswordPopup, shared::level::Level};

#[derive(Component)]
pub struct Password {
    secret: String,
    pub current_password: String,
}

impl Password {
    pub fn correct(&self) -> bool {
        self.secret == self.current_password
    }
}

pub fn create_password(
    commands: &mut Commands,
    level: &Single<&Level>,
) {
    if let Some(actual_string) = level.secret_password.clone() {
        commands.spawn(
            (
                Password { secret: actual_string, current_password: String::from("") },
                CleanupPasswordPopup,
            )
        );
    }
}

pub fn setup_password(
    mut commands: Commands,
    level: Single<&Level>,
) {
    create_password(&mut commands, &level);
}