use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
use std::time::Duration;


use crate::minigames::{
    shared::{level::Level, menu::menu_action::MenuAction, score::{
        Score,
        decrease_score,
        increase_score,
    }},
    shooting_game::{LossState, gun::{
        Gun, GunAnimationState, decrease_bullets
    }},
};

const TARGET_WIDTH_INTERVAL: (f32, f32) = (100., 200.);
const TARGET_HEIGHT_INTERVAL: (f32, f32) = (100., 200.);
const TARGET_Z_INDEX: f32 = 0.;

const RAILS_COUNT: u32 = 3;

const MIN_TARGET_LIFETIME: f32 = 3.;
const MAX_TARGET_LIFETIME: f32 = 10.;

const MIN_TARGET_SPEED: f32 = 100.;
const MAX_TARGET_SPEED: f32 = 400.;

const MAX_TARGET_COUNT: usize = 10;

const SWITCH_DIRECTION_PERCENTAGE: u32 = 99;

// const POSSIBLE_FRIENDS: [&str; 3] = ["shooting_game/green_atlas.png", "shooting_game/green_atlas.png", "shooting_game/green_atlas.png"];
// const POSSIBLE_ENEMIES: [&str; 3] = ["shooting_game/red_atlas.png", "shooting_game/red_atlas.png", "shooting_game/red_atlas.png"];

const POSSIBLE_FRIENDS: [&str; 3] = ["shooting_game/tests/green1_atlas.png", "shooting_game/tests/green2_atlas.png", "shooting_game/tests/green3_atlas.png"];
const POSSIBLE_ENEMIES: [&str; 3] = ["shooting_game/tests/red1_atlas.png", "shooting_game/tests/red2_atlas.png", "shooting_game/tests/red3_atlas.png"];

#[derive(Eq, PartialEq, Debug)]
pub enum TargetDirection {
    Right,
    Left,
}

#[derive(Eq, PartialEq)]
pub enum TargetAnimationStatus {
    None,
    External,
    Internal,
}

#[derive(Component)]
pub struct Target {
    pub current_x: f32,
    pub current_y: f32,
    width: f32,
    height: f32,
    current_direction: TargetDirection,
    movement_speed: f32,
    remaining_lifetime: f32,
    friendly: bool,
    is_animating: TargetAnimationStatus,
    should_be_deleted: bool,
}

impl Target {
    fn within_self_bounds(&self, x: f32, y: f32) -> bool {
        self.current_x - (self.width / 2.) <= x &&
            x <= self.current_x + (self.width / 2.) &&
            self.current_y - (self.height / 2.) <= y &&
            y <= self.current_y + (self.height / 2.)
    }

    fn random_anything(lwr_limit: u32, upr_limit: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(lwr_limit..upr_limit)
    }

    fn random_anything_f32(lwr_limit: f32, upr_limit: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(lwr_limit..upr_limit)
    }

    pub fn random_direction() -> TargetDirection {
        match Self::random_anything(0,2) {
            0 => TargetDirection::Left,
            1 => TargetDirection::Right,
            _ => TargetDirection::Left, // impossible to get to lol
        }
    }

    pub fn random_target_height() -> f32 {
        Self::random_anything_f32(TARGET_HEIGHT_INTERVAL.0, TARGET_HEIGHT_INTERVAL.1)
    }

    pub fn random_target_width() -> f32 {
        Self::random_anything_f32(TARGET_WIDTH_INTERVAL.0, TARGET_WIDTH_INTERVAL.1)
    }

    pub fn random_x_location(max_width: f32, target_width: f32) -> f32 {
        let half = (max_width - target_width) / 2.0;

        let value = Self::random_anything(0, (half * 2.0) as u32) as f32;

        value - half
    }

    pub fn random_y_location(max_height: f32) -> f32 {
        let rail_index = Self::random_anything(0, RAILS_COUNT) as f32;

        let spacing = max_height / (RAILS_COUNT as f32 + 1.5);
        let half = max_height / 2.0;

        -half + spacing * (rail_index + 2.0)
    }

    pub fn random_lifetime() -> f32 {
        Self::random_anything(MIN_TARGET_LIFETIME as u32, MAX_TARGET_LIFETIME as u32) as f32
    }

    pub fn random_speed() -> f32 {
        Self::random_anything(MIN_TARGET_SPEED as u32, MAX_TARGET_SPEED as u32) as f32
    }

    pub fn random_friendly() -> bool {
        Self::random_anything(0,10) >= 8
    }

    pub fn random_friend() -> String {
        String::from(POSSIBLE_FRIENDS[Self::random_anything(0, 3) as usize])
    }

    pub fn random_enemy() -> String {
        String::from(POSSIBLE_ENEMIES[Self::random_anything(0, 3) as usize])
    }
}

#[derive(Component)]
pub struct TargetCleanup;

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

pub fn spawn_individual_target(
    window: &Query<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok(window) = window.single() {
        let max_width = window.resolution.width();
        let max_height = window.resolution.height();
        let target_width = Target::random_target_width();
        let target_height = Target::random_target_height();
        let current_x = Target::random_x_location(max_width, target_width);
        let current_y = Target::random_y_location(max_height);
        let friendly = Target::random_friendly();
        let asset_string =
            if friendly {
                Target::random_friend()
            } else {
                Target::random_enemy()
            };

        let texture = asset_server.load(&asset_string);
        let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 10, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let target_animation_config = AnimationConfig::new(0, 9, 45);

        commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: target_animation_config.first_sprite_index,
                }),
                custom_size: Some(Vec2::new(target_width, target_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(current_x, current_y, TARGET_Z_INDEX),
            Target {
                current_x,
                current_y,
                width: target_width,
                height: target_height,
                current_direction: Target::random_direction(),
                movement_speed: Target::random_speed(),
                remaining_lifetime: Target::random_lifetime(),
                friendly: friendly,
                is_animating: TargetAnimationStatus::External,
                should_be_deleted: false,
            },
            TargetCleanup,
            target_animation_config,
        ));
    }
}

pub fn maintain_intended_target_count(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    targets_query: Query<&Target>,
) {
    for _ in targets_query.iter().len()..MAX_TARGET_COUNT {
        if Target::random_anything(0, 100) >= 98 {
            spawn_individual_target(&window, &mut commands, &asset_server, &mut texture_atlas_layouts);
        }
    }
}

pub fn advance_expire_and_despawn(
    time: Res<Time>,
    mut commands: Commands,
    targets_query: Query<(&mut Target, Entity)>, 
) {
    let passed_time = time.delta_secs();
    for (mut target, entity) in targets_query {
        if target.remaining_lifetime - passed_time < 0. {
            commands.entity(entity).despawn();
        } else {
            target.remaining_lifetime -= passed_time;
        }
    }

}

pub fn move_targets(
    time: Res<Time>,
    window: Single<& Window>,
    targets_query: Query<(&mut Transform, &mut Target)>
) {
    let passed_time = time.delta_secs();
    for (mut transform, mut target) in targets_query {
        let max_width = (window.resolution.width() - target.width) / 2.;
        let random_value = Target::random_anything(0, 100);
        if random_value >= SWITCH_DIRECTION_PERCENTAGE {
            target.current_direction =
                match target.current_direction {
                    TargetDirection::Left => TargetDirection::Right,
                    TargetDirection::Right => TargetDirection::Left,
                };
        }
        match target.current_direction {
            TargetDirection::Left => {
                let new_x = transform.translation.x - passed_time * target.movement_speed;
                if new_x < - max_width {
                    transform.translation.x = - max_width;
                    target.current_direction = TargetDirection::Right;
                } else {
                    transform.translation.x = new_x;
                }
            }
            TargetDirection::Right => {
                let new_x = transform.translation.x + passed_time * target.movement_speed;
                if new_x > max_width {
                    transform.translation.x = max_width;
                    target.current_direction = TargetDirection::Left;
                } else {
                    transform.translation.x = new_x;
                }
            }
        };
        target.current_x = transform.translation.x;
        target.current_y = transform.translation.y;
        
    }
}

pub fn listen_for_shots_in_target(
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<MouseButton>>,
    mut score: Single<&mut Score>,
    level: Single<&Level>,
    targets_query: Query<&mut Target>,
    mut gun: Single<&mut Gun>,
    mut menu_action_state: ResMut<NextState<MenuAction>>,
    mut loss_state: ResMut<NextState<LossState>>,
    mut gun_animation_state: ResMut<NextState<GunAnimationState>>,
) {
    if keys.just_pressed(MouseButton::Left) {
        gun_animation_state.set(GunAnimationState::External);
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);
            for mut target in targets_query {
                if target.within_self_bounds(x, y) && !target.should_be_deleted {
                    if target.friendly {
                        decrease_score(&mut score);
                    } else {
                        increase_score(&mut score);
                    }
                    target.is_animating = TargetAnimationStatus::Internal;
                    target.should_be_deleted = true;
                }
            }
        }
        if score.0 >= level.target_score {
            menu_action_state.set(MenuAction::PreWin);
        }
        if decrease_bullets(&mut gun).is_none() {
            menu_action_state.set(MenuAction::PreLose);
            loss_state.set(LossState::Bullets);
        }
    }
}

pub fn animate_target(
    time: Res<Time>,
    targets_query: Query<(&mut AnimationConfig, &mut Sprite, &mut Target)>,
) {
    for (mut config, mut sprite, mut target) in targets_query {
        match target.is_animating {
            TargetAnimationStatus::External => {
                config.frame_timer.tick(time.delta());
                if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index {
                        target.is_animating = TargetAnimationStatus::None;
                    } else {
                        atlas.index += 1;
                    }
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            },
            TargetAnimationStatus::Internal => {
                config.frame_timer.tick(time.delta());
                if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.first_sprite_index {
                        target.is_animating = TargetAnimationStatus::None;
                    } else {
                        atlas.index -= 1;
                    }
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            },
            TargetAnimationStatus::None => ()
        }
    }
}

pub fn remove_soft_deleted_targets(
    mut commands: Commands,
    targets_query: Query<(&Target, Entity)>,
) {
    for (target, entity) in targets_query {
        if target.should_be_deleted && target.is_animating == TargetAnimationStatus::None {
            commands.entity(entity).despawn();
        }
    }
}

pub fn reset_targets(
    // clean-up
    commands: &mut Commands,
    cleanup_entities: &Query<(Entity, &TargetCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}

pub fn cleanup_targets(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &TargetCleanup)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}