use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

mod custom_window_setup_plugin;
use custom_window_setup_plugin::CustomWindowSetupPlugin;

mod setup_room;
use setup_room::setup_room;

mod setup_movable_block;
use setup_movable_block::{setup_controllable_block, keyboard_input}; 

fn main() {
    App::new()
        .add_plugins(CustomWindowSetupPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_room)
        .add_systems(Startup, setup_controllable_block)
        .add_systems(Update, (keyboard_input))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}
