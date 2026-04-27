use bevy::{
    prelude::*,
    window::{PrimaryWindow, CursorIcon, CustomCursor, CustomCursorImage, CursorOptions, WindowFocused},
};

use crate::{camera::MainCamera, minigames::shared::level::Level};

const GUN_HEIGHT: f32 = 400.0;
const GUN_WIDTH: f32 = 100.0;
const GUN_Z_INDEX: f32 = 1.;

#[derive(Component)]
pub struct Gun {
    pub bullets: usize,
}

#[derive(Component)]
pub struct GunCleanup;

pub fn setup_cursor_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    commands.entity(*window).insert((
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: asset_server.load("shooting_game/crosshair.png"),
            texture_atlas: None,
            flip_x: false,
            flip_y: false,
            rect: None,
            // The hotspot is the point in the cursor image that will be
            // positioned at the mouse cursor's position.
            hotspot: (25, 25),
        })),
    ));
}

pub fn hide_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
}

pub fn show_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
}

pub fn create_gun(
    window: &Query<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    level: &Single<&Level>,
) {
    if let Ok(window) = window.single() {
        let height = window.resolution.height();
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                GUN_WIDTH,
                GUN_HEIGHT,
            ))),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.35))),
            Transform::from_xyz(0., -height/2., GUN_Z_INDEX),
            Gun { bullets: level.bullets.unwrap() },
            GunCleanup,
        ));
    }
}

pub fn setup_gun(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    level: Single<&Level>,
) {
    create_gun(&window, &mut commands, &mut materials, &mut meshes, &level);
}

pub fn gun_follows_mouse(
    mut query: Query<&mut Transform, With<Gun>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {

    if let Ok(window) = windows.single() {
        if let Ok((camera, camera_transform)) = camera_q.single() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
            {
                // Get the player translation in 2D
    
                for mut gun_transform in &mut query {
                    // Get the vector from the enemy ship to the player ship in 2D and normalize it.
                    let to_player = (world_position - gun_transform.translation.xy()).normalize();

                    // Get the quaternion to rotate from the initial enemy facing direction to the direction
                    // facing the player
                    let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

                    // Rotate the enemy to face the player
                    gun_transform.rotation = rotate_to_player;
                }
            }
        }
    }
}

pub fn decrease_bullets(gun_mut_ref: &mut Single<&mut Gun>) -> Option<usize> {
    if gun_mut_ref.bullets > 0 {
        gun_mut_ref.bullets -= 1;
        Some(gun_mut_ref.bullets)
    } else {
        None
    }
}

pub fn reset_gun(
    // shared
    commands: &mut Commands,
    // creation
    window: &Query<&Window, With<PrimaryWindow>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    level: &Single<&Level>,
    // cleanup
    cleanup_entities: &Query<(Entity, &GunCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
    create_gun(window, commands, materials, meshes, &level);
}

pub fn cleanup_gun(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &GunCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}