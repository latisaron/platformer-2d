use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

const WALL_THICKNESS: f32 = 10.0;

pub fn setup_room(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Single<& Window>,
) {
    let room_width = window.resolution.width();
    let room_height = window.resolution.height();

    let room_width_pos = (room_width - WALL_THICKNESS) / 2.0;
    let room_height_pos = (room_height - WALL_THICKNESS) / 2.0;

    let wall_coordinates: [((f32, f32, f32), (f32, f32)); 4] = [
        ((-room_width_pos, 0.0, 0.0), (WALL_THICKNESS, room_height)),
        ((room_width_pos, 0.0, 0.0), (WALL_THICKNESS, room_height)),
        ((0.0, -room_height_pos, 0.0), (room_width, WALL_THICKNESS)),
        ((0.0, room_height_pos, 0.0), (room_width, WALL_THICKNESS)),
    ];

    for ((x_pos, y_pos, z_pos), (x_size, y_size)) in &wall_coordinates {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(*x_size, *y_size))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(*x_size / 2., *y_size / 2.))
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Transform::from_xyz(*x_pos, *y_pos, *z_pos))
        .insert(Ccd::enabled());
    }
}