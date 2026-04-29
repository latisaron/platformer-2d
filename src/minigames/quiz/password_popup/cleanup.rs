use bevy::{prelude::*};

#[derive(Component)]
pub struct CleanupPasswordPopup;

pub fn cleanup_password_popup_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<CleanupPasswordPopup>>,
) {
    for entity in entities_query {
        commands.entity(entity).despawn();
    }
}