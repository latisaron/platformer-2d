use bevy::{prelude::*};

use crate::minigames::{knife_game::{
    BACKGROUND_Z_INDEX, CHOPPING_BLOCK_HEIGHT_PERCENTAGE, CHOPPING_BLOCK_WIDTH_PERCENTAGE, CUTTABLE_Z_INDEX, IMAGE_HEIGHT, IMAGE_WIDTH
}, shared::level::Level};

#[derive(Component)]
pub struct ChoppingBlock{
    pub width: f32,
    pub center: f32,
}

#[derive(Component)]
pub struct CleanupChoppingBlock;

pub fn create_chopping_block(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    chopping_block_width: f32,
    chopping_block_center: f32,
    chopping_block_height: f32,
    reference_width: f32,
    level: usize,
) {
    let image_center = ((chopping_block_center + reference_width  / 2.) / reference_width) * IMAGE_WIDTH; 
    let image_height = IMAGE_HEIGHT;
    let image_width = (chopping_block_width / reference_width) * IMAGE_WIDTH;
    
    let image_path = match level {
        1 => "knife_game/ant.png",
        2 => "knife_game/dark_ant.png",
        _ => "knife_game/final_boss_ant.png",
    };

    commands.spawn((
        Sprite {
            image: asset_server.load(image_path),
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
        CleanupChoppingBlock,
    ));
}

pub fn setup_chopping_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
    level: Single<& Level>,
) {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let chopping_block_width = window_width * CHOPPING_BLOCK_WIDTH_PERCENTAGE;
    let chopping_block_center = 0.;
    let chopping_block_height = window_height * CHOPPING_BLOCK_HEIGHT_PERCENTAGE;

    create_chopping_block(
        &mut commands,
        &mut asset_server,
        chopping_block_width,
        chopping_block_center,
        chopping_block_height,
        chopping_block_width,
        level.current_value,
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
        CleanupChoppingBlock,
    ));
}

pub fn cleanup_chopping_block(
    mut commands: Commands,
    cleanup_entities: Query<(Entity, &CleanupChoppingBlock)>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }
}

pub fn reset_chopping_block(
    mut commands: &mut Commands,
    mut asset_server: &mut ResMut<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    window: &Single<& Window>,
    cleanup_entities: Query<(Entity, &CleanupChoppingBlock)>,
    level: Single<& Level>,
) {
    for entities in cleanup_entities {
        commands.entity(entities.0).despawn();
    }

    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let chopping_block_width = window_width * CHOPPING_BLOCK_WIDTH_PERCENTAGE;
    let chopping_block_center = 0.;
    let chopping_block_height = window_height * CHOPPING_BLOCK_HEIGHT_PERCENTAGE;

    create_chopping_block(
        &mut commands,
        &mut asset_server,
        chopping_block_width,
        chopping_block_center,
        chopping_block_height,
        chopping_block_width,
        level.current_value,
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
        CleanupChoppingBlock,
    ));
}