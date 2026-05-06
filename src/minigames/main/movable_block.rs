use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::minigames::main::cleanup::MainCleanup;

const SPEED: f32 = 150.0;

const FRAME_W: u32 = 301;
const FRAME_H: u32 = 504;
const FRAME_COUNT: u32 = 4;
const ANIM_FPS: f32 = 8.0;

const FOOT_RADIUS: f32 = 20.0;
const FOOT_Y_OFFSET: f32 = -200.0; // negative = down, tune this

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct PlayerAtlases {
    pub left:  Handle<Image>,
    pub right: Handle<Image>,
    pub front: Handle<Image>,
    pub back:  Handle<Image>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, (keyboard_input, update_animation).chain());
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_W, FRAME_H),
        FRAME_COUNT,
        1,
        None,
        None,
    ));

    let atlases = PlayerAtlases {
        left:  asset_server.load("main/compact_left.png"),
        right: asset_server.load("main/compact_right.png"),
        front: asset_server.load("main/compact_back.png"),
        back:  asset_server.load("main/compact_front.png"),
    };

    let initial_image = atlases.front.clone();

    commands.spawn((
        Sprite {
            image: initial_image,
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
        RigidBody::Dynamic,
        Velocity::zero(),
        Damping { linear_damping: 12.0, angular_damping: 0.0 },
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Ccd::enabled(),
        Sleeping::disabled(),
        Direction::Down,
        AnimationTimer(Timer::from_seconds(1.0 / ANIM_FPS, TimerMode::Repeating)),
        atlases,
        MainCleanup,
    )).with_children(|parent| {
        parent.spawn((
            Collider::ball(FOOT_RADIUS),
            Transform::from_xyz(0.0, FOOT_Y_OFFSET, 0.0),
            MainCleanup,
        ));
    });
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Direction, &mut Velocity)>,
) {
    let Ok((mut dir, mut vel)) = query.single_mut() else { return };

    vel.linvel = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowRight) {
        *dir = Direction::Right;
        vel.linvel.x = SPEED;
    } else if keys.pressed(KeyCode::ArrowLeft) {
        *dir = Direction::Left;
        vel.linvel.x = -SPEED;
    } else if keys.pressed(KeyCode::ArrowUp) {
        *dir = Direction::Up;
        vel.linvel.y = SPEED;
    } else if keys.pressed(KeyCode::ArrowDown) {
        *dir = Direction::Down;
        vel.linvel.y = -SPEED;
    }
}

pub fn update_animation(
    time: Res<Time>,
    mut query: Query<(
        &Direction,
        &PlayerAtlases,
        &mut Sprite,
        &mut AnimationTimer,
        &Velocity,
    )>,
) {
    let Ok((dir, atlases, mut sprite, mut timer, vel)) = query.single_mut() else {
        return;
    };

    let target_image = match dir {
        Direction::Left  => &atlases.left,
        Direction::Right => &atlases.right,
        Direction::Up    => &atlases.back,
        Direction::Down  => &atlases.front,
    };

    if sprite.image != *target_image {
        sprite.image = target_image.clone();
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = 0;
        }
    }

    let is_moving = vel.linvel.length_squared() > 1.0;

    if is_moving {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % FRAME_COUNT as usize;
            }
        }
    } else {
        timer.0.reset();
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = 0;
        }
    }
}