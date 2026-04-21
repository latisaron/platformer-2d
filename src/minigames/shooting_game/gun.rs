use bevy::{
    prelude::*,
    window::{CursorIcon, CustomCursor, CustomCursorImage, CursorOptions},
};

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