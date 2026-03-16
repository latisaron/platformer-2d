use bevy::{prelude::*};
use std::time::Duration;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChoppingGameState {
    // Start,
    // Restart,
    // Dead,
    Playing,
    Cutting
}

#[derive(Component)]
pub struct ChoppingBlock{
    width: f32,
    center: f32,
}

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Knife;

#[derive(Component)]
pub struct Shadow;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}


const CHOPPING_BLOCK_WIDTH_PERCENTAGE: f32 = 0.7;
const CHOPPING_BLOCK_HEIGHT_PERCENTAGE: f32 = 0.3;
const BACKGROUND_Z_INDEX: f32 = 0.;
const CUTTABLE_Z_INDEX: f32 = 1.;
const MOVABLE_Z_INDEX: f32 = 2.;

const KNIFE_WIDTH: f32 = 50.;
const KNIFE_X_OFFSET_TO_SHADOW: f32 = 25.0;

const SHADOW_START_X_POSITION_PERCENTAGE: f32 = 0.35;
const SHADOW_START_Y_POSITION: f32 = 0.;
const SHADOW_HEIGHT_PERNCETAGE: f32 = 0.3;
const SHADOW_WIDTH: f32 = 3.;

const MOVEMENT_SPEED: f32 = 400.;

const IMAGE_WIDTH: f32 = 200.;
const IMAGE_HEIGHT: f32 = 80.;


#[derive(PartialEq, Eq, Debug)]
enum KnifeDirection {
    Down,
    Up,
    Stationary,
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
    direction: KnifeDirection,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            direction: KnifeDirection::Stationary,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn create_chopping_block(
    commands: &mut Commands,
    asset_server: &ResMut<AssetServer>,
    chopping_block_width: f32,
    chopping_block_center: f32,
    chopping_block_height: f32,
    reference_width: f32,
) {
    let image_center = ((chopping_block_center + reference_width  / 2.) / reference_width) * IMAGE_WIDTH; 
    let image_height = IMAGE_HEIGHT;
    let image_width = (chopping_block_width / reference_width) * IMAGE_WIDTH;

    commands.spawn((
        Sprite {
            image: asset_server.load("knife_game/pula.png"),
            rect: Some(Rect {
                min: Vec2::new(
                    image_center - image_width / 2.,
                    0.,
                ),
                max: Vec2::new(
                    image_center + image_width / 2.,
                    image_height,
                )
            }),
            custom_size: Some(Vec2::new(chopping_block_width, chopping_block_height)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(chopping_block_center, 0.0, CUTTABLE_Z_INDEX),
        ChoppingBlock { width: chopping_block_width, center: chopping_block_center },
    ));
}

pub fn setup_chopping_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
) {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let chopping_block_width = window_width * CHOPPING_BLOCK_WIDTH_PERCENTAGE;
    let chopping_block_center = 0.;
    let chopping_block_height = window_height * CHOPPING_BLOCK_HEIGHT_PERCENTAGE;

    create_chopping_block(
        &mut commands,
        &asset_server,
        chopping_block_width,
        chopping_block_center,
        chopping_block_height,
        chopping_block_width,
    );

    let background_width = chopping_block_width * 1.2;
    let background_height = chopping_block_height * 1.5;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            background_width,
            background_height,
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.82, 0.60, 0.35))),
        Transform::from_xyz(0., 0.0, BACKGROUND_Z_INDEX),
    ));
}

pub fn setup_knife(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
) {
    let width = window.resolution.width();
    let height = window.resolution.height();

    let shadow_x_start_position = - width * SHADOW_START_X_POSITION_PERCENTAGE ;
    let shadow_y_start_position = SHADOW_START_Y_POSITION;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            SHADOW_WIDTH,
            height * SHADOW_HEIGHT_PERNCETAGE,
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(shadow_x_start_position, shadow_y_start_position, MOVABLE_Z_INDEX),
        Movable{},
        Shadow{},
        Direction::Right,
    ));

    let knife_x_start_position = shadow_x_start_position + KNIFE_X_OFFSET_TO_SHADOW;
    let knife_y_start_position = - height * 0.1;

    let texture = asset_server.load("knife_game/knife_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 200), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let knife_animation_config = AnimationConfig::new(0, 9, 30);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: knife_animation_config.first_sprite_index,
            }),
            custom_size: Some(Vec2::new(KNIFE_WIDTH, height * (SHADOW_HEIGHT_PERNCETAGE + 0.2))),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(knife_x_start_position, knife_y_start_position, MOVABLE_Z_INDEX),
        Movable{},
        Knife{},
        Direction::Right,
        knife_animation_config,
    ));
}

pub fn move_objects(
    time: Res<Time>,
    window: Single<& Window>,
    mut knife: Single<(&mut Transform, &mut Direction), (With<Knife>, Without<Shadow>)>,
    mut shadow: Single<(&mut Transform, &mut Direction), (With<Shadow>, Without<Knife>)>,
) {
    let width = window.resolution.width();
    let passed_time = time.delta_secs();

    let shadow_upr_limit = width / 2. * CHOPPING_BLOCK_WIDTH_PERCENTAGE;
    let shadow_lwr_limit = - shadow_upr_limit;

    match *shadow.1 {
        Direction::Right => {
            let new_x_position = shadow.0.translation.x + passed_time * MOVEMENT_SPEED;
            if new_x_position >= shadow_upr_limit {
               shadow.0.translation.x = shadow_upr_limit;
               knife.0.translation.x = shadow_upr_limit + KNIFE_X_OFFSET_TO_SHADOW;
                *shadow.1 = Direction::Left;
                *knife.1 = Direction::Left;
            } else {
                shadow.0.translation.x += passed_time * MOVEMENT_SPEED;
                knife.0.translation.x += passed_time * MOVEMENT_SPEED;
            }
        },
        Direction::Left => {
            let new_x_position = shadow.0.translation.x - passed_time * MOVEMENT_SPEED;
            if new_x_position <= shadow_lwr_limit {
               shadow.0.translation.x = shadow_lwr_limit;
               knife.0.translation.x = shadow_lwr_limit + KNIFE_X_OFFSET_TO_SHADOW;
                *shadow.1 = Direction::Right;
                *knife.1 = Direction::Right;
            } else {
                shadow.0.translation.x -= passed_time * MOVEMENT_SPEED;
                knife.0.translation.x -= passed_time * MOVEMENT_SPEED;
            }
        }
    }
        
}

pub fn register_keystroke(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<ChoppingGameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(ChoppingGameState::Cutting);
    }
}

pub fn cut_animation(
    time: ResMut<Time<Virtual>>,
    mut next_state: ResMut<NextState<ChoppingGameState>>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
    chopping_block: Single<(& ChoppingBlock, Entity)>,
    shadow: Single<&Transform, (With<Shadow>, Without<Knife>)>,
    knife: Single<(&mut AnimationConfig, &mut Sprite), (With<Knife>, Without<Shadow>)>,
) { 
    let (mut config, mut sprite) = knife.into_inner();
    config.frame_timer.tick(time.delta());
    if config.direction == KnifeDirection::Stationary {
        config.direction = KnifeDirection::Down;
    } else if config.direction == KnifeDirection::Down {
        if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index == config.last_sprite_index {
                config.direction = KnifeDirection::Up;

                let chopping_block_lwr = chopping_block.0.center - chopping_block.0.width / 2.;
                let chopping_block_upr = chopping_block.0.center + chopping_block.0.width / 2.;
                let shadow_x_pos = shadow.translation.x;

                if shadow_x_pos < chopping_block_lwr || shadow_x_pos > chopping_block_upr {
                    // game over, not impl so far
                } else {
                    let left_side_diff = shadow_x_pos - chopping_block_lwr;
                    let right_side_diff = chopping_block_upr - shadow_x_pos;
                    let (new_center, new_width) = 
                        if right_side_diff > left_side_diff {
                            ((shadow_x_pos + chopping_block_lwr) / 2., left_side_diff)
                        } else {
                            ((shadow_x_pos + chopping_block_upr) / 2., right_side_diff)
                        };

                    

                    commands.entity(chopping_block.1).despawn();
                    create_chopping_block(
                        &mut commands,
                        &asset_server,
                        new_width,
                        new_center,
                        window.resolution.height() * CHOPPING_BLOCK_HEIGHT_PERCENTAGE,
                        window.resolution.width() * CHOPPING_BLOCK_WIDTH_PERCENTAGE,
                    );
                }
            } else {
                atlas.index += 1;
            }
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }
    } else {
        if config.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index == config.first_sprite_index {
                config.direction = KnifeDirection::Stationary;
                next_state.set(ChoppingGameState::Playing);
            } else {
                atlas.index -= 1;
            }
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }
    }
}