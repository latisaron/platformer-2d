use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

const TARGET_WIDTH: f32 = 100.;
const TARGET_HEIGHT: f32 = 200.;
const TARGET_Z_INDEX: f32 = 0.;

const RAILS_COUNT: u32 = 3;

const MIN_TARGET_LIFETIME: f32 = 3.;
const MAX_TARGET_LIFETIME: f32 = 10.;

const MIN_TARGET_SPEED: f32 = 200.;
const MAX_TARGET_SPEED: f32 = 800.;

const MAX_TARGET_COUNT: usize = 8;

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
    current_direction: TargetDirection,
    movement_speed: f32,
    remaining_lifetime: f32,
}

impl Target {
    fn random_anything(lwr_limit: u32, upr_limit: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(lwr_limit..upr_limit)
    }

    pub fn random_direction() -> TargetDirection {
        match (Self::random_anything(0,2)) {
            0 => TargetDirection::Left,
            1 => TargetDirection::Right,
            _ => TargetDirection::Left, // impossible to get to lol
        }
    }

    pub fn random_x_location(max_width: f32) -> f32 {
        let half = (max_width - TARGET_WIDTH) / 2.0;

        let value = Self::random_anything(0, (half * 2.0) as u32) as f32;

        value - half
    }

    pub fn random_y_location(max_height: f32) -> f32 {
        let rail_index = Self::random_anything(0, RAILS_COUNT) as f32;

        let spacing = max_height / (RAILS_COUNT as f32 + 1.0);
        let half = max_height / 2.0;

        -half + spacing * (rail_index + 1.0)
    }

    pub fn random_lifetime() -> f32 {
        Self::random_anything(MIN_TARGET_LIFETIME as u32, MAX_TARGET_LIFETIME as u32) as f32
    }

    pub fn random_speed() -> f32 {
        Self::random_anything(MIN_TARGET_SPEED as u32, MAX_TARGET_SPEED as u32) as f32
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
        let current_x = Target::random_x_location(max_width);
        let current_y = Target::random_y_location(max_height);

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                TARGET_WIDTH,
                TARGET_HEIGHT,
            ))),
            MeshMaterial2d(materials.add(Color::srgb(0.82, 0.60, 0.35))),
            Transform::from_xyz(current_x, current_y, TARGET_Z_INDEX),
            Target {
                current_x,
                current_y,
                current_direction: Target::random_direction(),
                movement_speed: Target::random_speed(),
                remaining_lifetime: Target::random_lifetime(),
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
    let max_width = (window.resolution.width() - TARGET_WIDTH) / 2.;
    let passed_time = time.delta_secs();
    for (mut transform, mut target) in targets_query {
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
        
    }
}