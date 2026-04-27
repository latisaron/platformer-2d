use bevy::{
    prelude::*,
};

use crate::minigames::{shared::level::Level, shooting_game::gun::Gun};

#[derive(Component)]
pub struct BulletsText;

#[derive(Component)]
pub struct BulletsTextCleanup;

pub fn setup_bullets(
    mut commands: Commands,
    level: Single<&Level>,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(5),
            ..default()
        },
        BulletsTextCleanup
    )).with_children(|parent| {
        parent.spawn((
            Text::default(),
            TextLayout::new_with_justify(Justify::Center),
            TextShadow::default(),
        )).with_children(|parent| {
            parent.spawn((
                TextSpan::new(format!("Bullets: {} / {}", level.bullets.unwrap(), level.bullets.unwrap())),
                BulletsText,
            ));
        });
    });
}

pub fn display_bullets(
    gun: Single<&Gun>,
    level: Single<&Level>,
    mut query: Query<&mut TextSpan, With<BulletsText>>,
) {
    let mut span = query.single_mut().unwrap();
    span.0 = format!("Bullets: {} / {}", gun.bullets, level.bullets.unwrap());
}

pub fn cleanup_bullets_text(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &BulletsTextCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}