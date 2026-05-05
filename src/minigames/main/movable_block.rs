use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

const SPEED: f32 = 50.0;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
pub fn setup_controllable_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(50.0, 50.0))
    .insert(Restitution {
        coefficient: 0.0,
        combine_rule: CoefficientCombineRule::Min,
    })
    .insert(Transform::from_xyz(0.0, 0.0, 0.0))
    .insert(Velocity::zero())
    .insert(Damping {
        linear_damping: 10.0,
        angular_damping: 10.0,
    })
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(GravityScale(0.0))
    .insert(Direction::Right);
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    block_direction: Single<(&mut Direction, &mut Velocity)>,
) {
    let (mut dir, mut vel) = block_direction.into_inner();
    if keys.pressed(KeyCode::ArrowRight) {
        *dir = Direction::Right;
        vel.linvel.x += SPEED;
    } else if keys.pressed(KeyCode::ArrowLeft) {
        *dir = Direction::Left;
        vel.linvel.x -= SPEED;
    } else if keys.pressed(KeyCode::ArrowUp) {
        *dir = Direction::Up;
        vel.linvel.y += SPEED;
    } else if keys.pressed(KeyCode::ArrowDown) {
        *dir = Direction::Down;
        vel.linvel.y -= SPEED;
    }
}