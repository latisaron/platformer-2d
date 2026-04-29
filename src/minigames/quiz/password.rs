use bevy::{prelude::*};

use crate::minigames::shared::level::Level;

#[derive(Component)]
pub struct Password {
    secret: String,
}

#[derive(Component)]
pub struct CleanupPassword;

pub fn create_password(
    commands: &mut Commands,
    level: &Single<&Level>,
) {
    if let Some(actual_string) = level.secret_password.clone() {
        commands.spawn(
            Password { secret: actual_string }
        );
    }
}

pub fn setup_password(
    mut commands: Commands,
    level: Single<&Level>,
) {
    create_password(&mut commands, &level);
}

pub fn restart_password(
    commands: &mut Commands,
    level: &Single<&Level>,
    // cleanup
    password_entities: &Query<Entity, With<CleanupPassword>>,
) {
    for entity in password_entities {
        commands.entity(entity).despawn();
    }
    create_password(commands, level);;
}

pub fn cleanup_password(
    mut commands: Commands,
    password_entities: &Query<Entity, With<CleanupPassword>>,
) {
    for entity in password_entities {
        commands.entity(entity).despawn();
    }
}