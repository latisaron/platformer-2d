use bevy::{window::WindowMode, prelude::*};

pub struct CustomWindowSetupPlugin;

impl Plugin for CustomWindowSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
                    ..default()
                }),
                ..default()
            }
        ));
    }
}