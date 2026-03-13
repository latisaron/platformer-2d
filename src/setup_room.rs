use bevy::{
    prelude::*,
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2};
use bevy_rapier2d::prelude::*;

const WALL_THICKNESS: f32 = 10.0;

pub fn setup_walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Single<& Window>,
) {
    let room_width = window.resolution.width();
    let room_height = window.resolution.height();

    let room_width_pos = (room_width - WALL_THICKNESS) / 2.0;
    let room_height_pos = (room_height - WALL_THICKNESS) / 2.0;

    let wall_coordinates: [((f32, f32, f32), (f32, f32)); 4] = [
        ((-room_width_pos, 0.0, 0.0), (WALL_THICKNESS, room_height)),
        ((room_width_pos, 0.0, 0.0), (WALL_THICKNESS, room_height)),
        ((0.0, -room_height_pos, 0.0), (room_width, WALL_THICKNESS)),
        ((0.0, room_height_pos, 0.0), (room_width, WALL_THICKNESS)),
    ];

    for ((x_pos, y_pos, z_pos), (x_size, y_size)) in &wall_coordinates {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(*x_size, *y_size))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(*x_size / 2., *y_size / 2.))
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Transform::from_xyz(*x_pos, *y_pos, *z_pos))
        .insert(Ccd::enabled());
    }
}

pub fn setup_bookshelf(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
) {
    let room_width = window.resolution.width();

    let book_shelf_x_size = 240.;
    let book_shelf_y_size = 918.;

    let book_shelf_draw_x_position  = (room_width - book_shelf_x_size - WALL_THICKNESS) / 2.;
    let book_shelf_draw_y_position = 0.;

    commands.spawn(
        (
            Sprite::from_image(asset_server.load("textures/book_shelf.png")),
            Transform::from_xyz(book_shelf_draw_x_position, book_shelf_draw_y_position, 1.),
            // Transform::from_xyz(0., 0., 0.),
            Collider::cuboid(book_shelf_x_size / 2., book_shelf_y_size / 2.),
            Restitution {coefficient: 0.0, combine_rule: CoefficientCombineRule::Min},
        )
    );
}

pub fn setup_couch(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
) {
    let room_width = window.resolution.width();

    let couch_x_size = 400.;
    let couch_y_size = 683.;

    let couch_draw_x_position  =  -(room_width / 2.0) + (couch_x_size - WALL_THICKNESS) / 2.;
    let couch_draw_y_position = 0.;

    commands.spawn(
        (
            Sprite::from_image(asset_server.load("textures/couch_good.png")),
            Transform::from_xyz(couch_draw_x_position, couch_draw_y_position, 1.),
            // Transform::from_xyz(0., 0., 0.),
            Collider::cuboid(couch_x_size / 2., couch_y_size / 2.),
            Restitution {coefficient: 0.0, combine_rule: CoefficientCombineRule::Min},
        )
    );
}

pub fn setup_drawer(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window: Single<& Window>,
) {
    let room_height = window.resolution.height();

    let drawer_x_size = 250.;
    let drawer_y_size = 250.;

    let drawer_draw_x_position  =  (drawer_x_size - WALL_THICKNESS) / 4.;
    let drawer_draw_y_position = (room_height - drawer_y_size) / 2.;

    commands.spawn(
        (
            Sprite::from_image(asset_server.load("textures/drawer.png")),
            Transform::from_xyz(drawer_draw_x_position, drawer_draw_y_position, 1.),
            // Transform::from_xyz(0., 0., 0.),
            Collider::cuboid(drawer_x_size / 2., drawer_y_size / 2.),
            Restitution {coefficient: 0.0, combine_rule: CoefficientCombineRule::Min},
        )
    );
}

pub fn setup_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<& Window>,
) {
    let room_height = window.resolution.height();
    let room_width = window.resolution.width();

    let floor_tile_size = 160.0;
    let floor_offset = floor_tile_size / 2. + 89.;

    let image_with_repeated_sampler = asset_server.load_with_settings(
        "textures/floor.png",
        |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    // rewriting mode to repeat image,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        },
    );

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(room_width, room_height))),
        MeshMaterial2d(
            materials.add(ColorMaterial {
                texture: Some(image_with_repeated_sampler),
                uv_transform: Affine2::from_scale(Vec2::new(room_width / floor_tile_size, room_height / floor_tile_size)),
                ..default()
            }),
        ),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        children![(
            Text2d::new("Repeat On"),
            Transform::from_xyz(0., floor_offset, 0.),
        )],

    ));
}