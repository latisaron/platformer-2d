use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::minigames::{
    shared::score::{
        Score,
        decrease_score,
        increase_score,
    },
    shooting_game::gun::{
        Gun,
        decrease_bullets,
    },
};

const TARGET_WIDTH_INTERVAL: (f32, f32) = (50., 100.);
const TARGET_HEIGHT_INTERVAL: (f32, f32) = (100., 200.);
const TARGET_Z_INDEX: f32 = 0.;

const RAILS_COUNT: u32 = 3;

const MIN_TARGET_LIFETIME: f32 = 3.;
const MAX_TARGET_LIFETIME: f32 = 10.;

const MIN_TARGET_SPEED: f32 = 100.;
const MAX_TARGET_SPEED: f32 = 400.;

const MAX_TARGET_COUNT: usize = 10;

const SWITCH_DIRECTION_PERCENTAGE: u32 = 99;

#[derive(Eq, PartialEq, Debug)]
pub enum TargetDirection {
    Right,
    Left,
}

#[derive(Component)]
pub struct Target {
    pub current_x: f32,
    pub current_y: f32,
    width: f32,
    height: f32,
    current_direction: TargetDirection,
    movement_speed: f32,
    remaining_lifetime: f32,
    friendly: bool,
}

impl Target {
    fn within_self_bounds(&self, x: f32, y: f32) -> bool {
        self.current_x - (self.width / 2.) <= x &&
            x <= self.current_x + (self.width / 2.) &&
            self.current_y - (self.height / 2.) <= y &&
            y <= self.current_y + (self.height / 2.)
    }

    fn random_anything(lwr_limit: u32, upr_limit: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(lwr_limit..upr_limit)
    }

    fn random_anything_f32(lwr_limit: f32, upr_limit: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(lwr_limit..upr_limit)
    }

    pub fn random_direction() -> TargetDirection {
        match Self::random_anything(0,2) {
            0 => TargetDirection::Left,
            1 => TargetDirection::Right,
            _ => TargetDirection::Left, // impossible to get to lol
        }
    }

    pub fn random_target_height() -> f32 {
        Self::random_anything_f32(TARGET_HEIGHT_INTERVAL.0, TARGET_HEIGHT_INTERVAL.1)
    }

    pub fn random_target_width() -> f32 {
        Self::random_anything_f32(TARGET_WIDTH_INTERVAL.0, TARGET_WIDTH_INTERVAL.1)
    }

    pub fn random_x_location(max_width: f32, target_width: f32) -> f32 {
        let half = (max_width - target_width) / 2.0;

        let value = Self::random_anything(0, (half * 2.0) as u32) as f32;

        value - half
    }

    pub fn random_y_location(max_height: f32) -> f32 {
        let rail_index = Self::random_anything(0, RAILS_COUNT) as f32;

        let spacing = max_height / (RAILS_COUNT as f32 + 1.5);
        let half = max_height / 2.0;

        -half + spacing * (rail_index + 2.0)
    }

    pub fn random_lifetime() -> f32 {
        Self::random_anything(MIN_TARGET_LIFETIME as u32, MAX_TARGET_LIFETIME as u32) as f32
    }

    pub fn random_speed() -> f32 {
        Self::random_anything(MIN_TARGET_SPEED as u32, MAX_TARGET_SPEED as u32) as f32
    }

    pub fn random_friendly() -> bool {
        Self::random_anything(0,10) >= 8
    }
}

pub fn spawn_individual_target(
    window: &Query<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    if let Ok(window) = window.single() {
        let max_width = window.resolution.width();
        let max_height = window.resolution.height();
        let target_width = Target::random_target_width();
        let target_height = Target::random_target_height();
        let current_x = Target::random_x_location(max_width, target_width);
        let current_y = Target::random_y_location(max_height);
        let friendly = Target::random_friendly();
        let color =
            if friendly {
                Color::srgb(0.0, 0.60, 0.35)
            } else {
                Color::srgb(0.6, 0.0, 0.35)
            };

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                target_width,
                target_height,
            ))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(current_x, current_y, TARGET_Z_INDEX),
            Target {
                current_x,
                current_y,
                width: target_width,
                height: target_height,
                current_direction: Target::random_direction(),
                movement_speed: Target::random_speed(),
                remaining_lifetime: Target::random_lifetime(),
                friendly: friendly,
            },
        ));
    }
}

pub fn maintain_intended_target_count(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    targets_query: Query<&Target>,
) {
    for _ in targets_query.iter().len()..MAX_TARGET_COUNT {
        if Target::random_anything(0, 100) >= 98 {
            spawn_individual_target(&window, &mut commands, &mut materials, &mut meshes);
        }
    }
}

pub fn advance_expire_and_despawn(
    time: Res<Time>,
    mut commands: Commands,
    targets_query: Query<(&mut Target, Entity)>, 
) {
    let passed_time = time.delta_secs();
    for (mut target, entity) in targets_query {
        if target.remaining_lifetime - passed_time < 0. {
            commands.entity(entity).despawn();
        } else {
            target.remaining_lifetime -= passed_time;
        }
    }

}

pub fn move_targets(
    time: Res<Time>,
    window: Single<& Window>,
    targets_query: Query<(&mut Transform, &mut Target)>
) {
    let passed_time = time.delta_secs();
    for (mut transform, mut target) in targets_query {
        let max_width = (window.resolution.width() - target.width) / 2.;
        let random_value = Target::random_anything(0, 100);
        if random_value >= SWITCH_DIRECTION_PERCENTAGE {
            target.current_direction =
                match target.current_direction {
                    TargetDirection::Left => TargetDirection::Right,
                    TargetDirection::Right => TargetDirection::Left,
                };
        }
        match target.current_direction {
            TargetDirection::Left => {
                let new_x = transform.translation.x - passed_time * target.movement_speed;
                if new_x < - max_width {
                    transform.translation.x = - max_width;
                    target.current_direction = TargetDirection::Right;
                } else {
                    transform.translation.x = new_x;
                }
            }
            TargetDirection::Right => {
                let new_x = transform.translation.x + passed_time * target.movement_speed;
                if new_x > max_width {
                    transform.translation.x = max_width;
                    target.current_direction = TargetDirection::Left;
                } else {
                    transform.translation.x = new_x;
                }
            }
        };
        target.current_x = transform.translation.x;
        target.current_y = transform.translation.y;
        
    }
}

pub fn listen_for_shots_in_target(
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut score: Single<&mut Score>,
    targets_query: Query<(&Target, Entity)>,
    mut gun: Single<&mut Gun>,
) {
    if keys.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);
            for (target, entity) in targets_query {
                if target.within_self_bounds(x, y) {
                    if target.friendly {
                        decrease_score(&mut score);
                    } else {
                        increase_score(&mut score);
                    }
                    commands.entity(entity).despawn();
                }
            }
        }
        if decrease_bullets(&mut gun).is_none() {
            // game over - not impl 
        }
    }
}

