use bevy::{prelude::*};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}
