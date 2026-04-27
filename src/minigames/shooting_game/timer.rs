use bevy::{prelude::*};
use crate::minigames::{shared::{level::Level, menu::menu_action::MenuAction}, shooting_game::LossState};

#[derive(Component)]
pub struct Timer {
    pub remaining: f32,
}

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct TimerCleanup;

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

pub fn create_timer(
    commands: &mut Commands,
    level: &Single<&Level>
) {
    let remaining = level.target_time.unwrap();
    commands.spawn((
        Timer { remaining },
        TimerCleanup,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            ..default()
        },
        TimerCleanup,
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

pub fn setup_timer(
    mut commands: Commands,
    level: Single<&Level>
) {
    create_timer(&mut commands, &level);
}

pub fn update_timer(
    time: Res<Time>,
    mut timer: Single<&mut Timer>,
    mut timer_text_query: Query<&mut TextSpan, With<TimerText>>,
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut loss_state: ResMut<NextState<LossState>>,
) {
    let passed_time = time.delta_secs();
    timer.subtract(passed_time);
    let mut span = timer_text_query.single_mut().unwrap();
    span.0 = format!("Remaining Time: {:.2}", timer.remaining);
    if timer.remaining == 0. {
        menu_action_state.set(MenuAction::PreLose);
        loss_state.set(LossState::Timer);
    }
}

pub fn reset_timer(
    // timer
    level: &Single<&Level>,
    // cleanup
    commands: &mut Commands,
    cleanup_entities: &Query<(Entity, &TimerCleanup)>,
) { 
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
    create_timer(commands, level);
}

pub fn cleanup_timer(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &TimerCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}