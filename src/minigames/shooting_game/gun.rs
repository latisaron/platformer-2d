use bevy::{
    prelude::*,
    window::{PrimaryWindow, CursorIcon, CustomCursor, CustomCursorImage, CursorOptions},
};
use std::time::Duration;

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

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

#[derive(States, PartialEq, Eq, Hash, Debug, Clone)]
pub enum GunAnimationState {
    None,
    External,
    Internal,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}


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
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    level: &Single<&Level>,
) {
    if let Ok(window) = window.single() {
        let height = window.resolution.height();

        let texture = asset_server.load("shooting_game/gun_atlas.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 200), 10, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let gun_animation_config = AnimationConfig::new(0, 9, 240);

        commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: gun_animation_config.first_sprite_index,
                }),
                custom_size: Some(Vec2::new(GUN_WIDTH, GUN_HEIGHT)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., -(height - GUN_HEIGHT / 2.)/2., GUN_Z_INDEX),
            Gun { bullets: level.bullets.unwrap() },
            GunCleanup,
            gun_animation_config,
        ));
    }
}

pub fn setup_gun(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level: Single<&Level>,
) {
    create_gun(&window, &mut commands, &asset_server, &mut texture_atlas_layouts, &level);
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
    if gun_mut_ref.bullets > 1 {
        gun_mut_ref.bullets -= 1;
        Some(gun_mut_ref.bullets)
    } else {
        gun_mut_ref.bullets -= 1;
        None
    }
}

pub fn animate_gun_out(
    time: Res<Time>,
    gun: Single<(&mut AnimationConfig, &mut Sprite), With<Gun>>,
    mut gun_animation_state: ResMut<NextState<GunAnimationState>>,
) {
    let (mut config, mut sprite) = gun.into_inner();
    config.frame_timer.tick(time.delta());
    if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index += 1;
        if atlas.index == config.last_sprite_index {
            gun_animation_state.set(GunAnimationState::Internal);
        }
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
    }
}

pub fn animate_gun_in(
    time: Res<Time>,
    gun: Single<(&mut AnimationConfig, &mut Sprite), With<Gun>>,
    mut gun_animation_state: ResMut<NextState<GunAnimationState>>,
) {
    let (mut config, mut sprite) = gun.into_inner();
    config.frame_timer.tick(time.delta());
    if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index -= 1;
        if atlas.index == config.first_sprite_index {
            gun_animation_state.set(GunAnimationState::None);
        }
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
    }
}

pub fn reset_gun(
    // shared
    commands: &mut Commands,
    // creation
    window: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    level: &Single<&Level>,
    // cleanup
    cleanup_entities: &Query<(Entity, &GunCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
    create_gun(window, commands, asset_server, texture_atlas_layouts, &level);
}

pub fn cleanup_gun(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &GunCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}