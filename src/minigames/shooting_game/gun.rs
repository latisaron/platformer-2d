use bevy::{
    prelude::*,
    window::{PrimaryWindow, CursorIcon, CustomCursor, CustomCursorImage, CursorOptions},
};

const GUN_HEIGHT: f32 = 400.0;
const GUN_WIDTH: f32 = 100.0;
const GUN_Z_INDEX: f32 = 1.;

#[derive(Component)]
pub struct Gun;

pub fn setup_cursor_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<Window>>,
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

pub fn setup_gun(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
            Gun
        ));
    }
}