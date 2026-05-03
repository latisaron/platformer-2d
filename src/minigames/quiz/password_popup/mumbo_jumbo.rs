use std::time::Duration;

use bevy::{prelude::*};

use crate::minigames::quiz::password_popup::{cleanup::CleanupPasswordPopup, mumbo_jumbo, popup::{POPUP_HEIGHT, POPUP_WIDTH, POPUP_X, POPUP_Y}};

#[derive(Component)]
pub struct MumboJumbo {
    up: bool,
}


#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
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

pub fn setup_mumbo_jumbo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("quiz_game/mumbo_jumbo_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 10), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mumbo_jumbo_animation_config = AnimationConfig::new(0, 9, 12);

    let mumbo_jumbo_width = 0.820 * POPUP_WIDTH;
    let mumbo_jumbo_height = 0.255 * POPUP_HEIGHT;

    let mumbo_jumbo_y = 0.162 * POPUP_HEIGHT;
    let mumbo_jumbo_x = 0.;

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: mumbo_jumbo_animation_config.first_sprite_index,
            }),
            custom_size: Some(Vec2::new(mumbo_jumbo_width, mumbo_jumbo_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(mumbo_jumbo_x, mumbo_jumbo_y, 6.),
        mumbo_jumbo_animation_config,
        CleanupPasswordPopup,
        MumboJumbo { up: true },
    ));
}

pub fn animate_mumbo_jumbo(
    time: Res<Time>,
    mj: Single<(&mut AnimationConfig, &mut Sprite, &mut MumboJumbo)>,
) {
    let (mut config, mut sprite, mut mumbo_jumbo) = mj.into_inner();
    config.frame_timer.tick(time.delta());
    if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
        if mumbo_jumbo.up {
            atlas.index += 1;
        } else {
            atlas.index -= 1;
        }
        if atlas.index == config.last_sprite_index {
            mumbo_jumbo.up = false;
        } else if atlas.index == config.first_sprite_index {
            mumbo_jumbo.up = true;
        }
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
    }
}
