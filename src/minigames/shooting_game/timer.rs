use bevy::{prelude::*};
use crate::minigames::shared::level::{Level};

#[derive(Component)]
pub struct Timer {
    pub remaining: f32,
}

#[derive(Component)]
pub struct TimerText;

impl Timer {
    pub fn done(&self) -> bool {
        self.remaining <= 0.
    }

    pub fn subtract(&mut self, b: f32) {
        self.remaining -= b;
        if self.remaining < 0. {
            self.remaining = 0.;
        }
    }
}

pub fn setup_timer(
    mut commands: Commands,
    level: Single<&Level>
) {
    let remaining = level.target_time.unwrap();
    commands.spawn((
        Timer { remaining },
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::default(),
            TextLayout::new_with_justify(Justify::Center),
            TextShadow::default(),
        )).with_children(|parent| {
            parent.spawn((
                TextSpan::new(format!("Remaining Time: {}", remaining)),
                TimerText,
            ));
        });
    });
}

pub fn update_timer(
    time: Res<Time>,
    mut timer: Single<&mut Timer>,
    mut timer_text_query: Query<&mut TextSpan, With<TimerText>>
) {
    let passed_time = time.delta_secs();
    timer.subtract(passed_time);
    let mut span = timer_text_query.single_mut().unwrap();
    span.0 = format!("Remaining Time: {:.2}", timer.remaining);
}