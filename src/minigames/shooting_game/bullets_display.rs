use bevy::{
    prelude::*,
};

use crate::minigames::{shared::level::Level, shooting_game::gun::Gun};

#[derive(Component)]
pub struct BulletsText;

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
