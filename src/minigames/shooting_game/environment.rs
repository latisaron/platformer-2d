use bevy::{prelude::*, window::PrimaryWindow};

// const ENVIRONMENT_ASSET: &'static str = "shooting_game/tests/environment.png";
const ENVIRONMENT_ASSET: &'static str = "shooting_game/environment.png";

#[derive(Component)]
pub struct EnvironmentCleanup;

pub fn create_environment(
    window: &Single<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load(ENVIRONMENT_ASSET),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 0.0, 0.),
        EnvironmentCleanup,
    ));
}

pub fn setup_environment(
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    create_environment(&window, &mut commands, &asset_server);
}


pub fn cleanup_environment(
    mut commands: Commands,
    cleanup_entities: Query<Entity, With<EnvironmentCleanup>>,
) {
    for entity in cleanup_entities {
        commands.entity(entity).despawn();
    }
}