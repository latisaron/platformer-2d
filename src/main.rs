use bevy::{window::WindowMode, prelude::*};
use bevy_rapier2d::prelude::*;

mod camera;
// use setup_room::{setup_walls, setup_bookshelf, setup_couch, setup_drawer, setup_floor};

// mod setup_movable_block;
// use setup_movable_block::{setup_controllable_block, keyboard_input};

// mod setup_chopping_block;
// use setup_chopping_block::{setup_chopping_block, setup_knife, move_objects, register_keystroke, cut_animation, ChoppingGameState};

use camera::setup_graphics;

mod minigames;
use minigames::knife_game::{KnifeMinigamePlugin};
use minigames::shared::menu::state_management::{GameState};

use crate::minigames::main::MainMinigamePlugin;
use crate::minigames::quiz::QuizMinigamePlugin;
use crate::minigames::shooting_game::ShootingMinigamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_plugins(MainMinigamePlugin)
        .add_plugins(KnifeMinigamePlugin)
        .add_plugins(ShootingMinigamePlugin)
        .add_plugins(QuizMinigamePlugin)
        .run();

        // .add_systems(Startup, setup_walls)
        // .add_systems(Startup, (setup_floor, setup_bookshelf, setup_couch, setup_drawer).chain())
        // .add_systems(Startup, setup_controllable_block)
        // .add_systems(Update, (keyboard_input))
        // .run();
}

