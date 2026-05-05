use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::minigames::main::cleanup::MainCleanup;

const WALL_THICKNESS: f32 = 10.0;

/// Converts top-left-origin percentage coords to Bevy center-origin world coords.
/// Returns (bevy_x, bevy_y, world_width, world_height)
fn pct_to_world(
    center_x_pct: f32,
    center_y_pct: f32,
    width_pct: f32,
    height_pct: f32,
    room_width: f32,
    room_height: f32,
) -> (f32, f32, f32, f32) {
    let x = (center_x_pct / 100.0) * room_width - room_width / 2.0;
    let y = room_height / 2.0 - (center_y_pct / 100.0) * room_height;
    let w = (width_pct / 100.0) * room_width;
    let h = (height_pct / 100.0) * room_height;
    (x, y, w, h)
}

fn spawn_solid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32, y: f32, w: f32, h: f32,
    z: f32,
    path: String,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load(&path),
            custom_size: Some(Vec2::new(w, h)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(x, y, z),
        Collider::cuboid(w / 2., h / 2.),
        Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Min },
        MainCleanup,
    ));
}

pub fn setup_walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();

    let rw_pos = (rw - WALL_THICKNESS) / 2.0;
    let rh_pos = (rh - WALL_THICKNESS) / 2.0;

    let walls: [((f32, f32, f32), (f32, f32)); 4] = [
        ((-rw_pos, 0.0, 0.0), (WALL_THICKNESS, rh)),
        (( rw_pos, 0.0, 0.0), (WALL_THICKNESS, rh)),
        ((0.0, -rh_pos, 0.0), (rw, WALL_THICKNESS)),
        ((0.0,  rh_pos, 0.0), (rw, WALL_THICKNESS)),
    ];

    for ((x, y, z), (w, h)) in &walls {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(*w, *h))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
            RigidBody::Fixed,
            Collider::cuboid(*w / 2., *h / 2.),
            Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Min },
            Transform::from_xyz(*x, *y, *z),
            Ccd::enabled(),
            MainCleanup,
        ));
    }
}

pub fn setup_bed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();
    // dropped center_y 31% → 36%
    let (x, y, w, h) = pct_to_world(12.9, 36.0, 24.9, 62.0, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/couch_gun.png"));
}

pub fn setup_drawer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();
    // flush with right wall: center_x = 100 - width/2 = 100 - 16.55 = 83.45%
    let (x, y, w, h) = pct_to_world(83.45, 11.0, 33.1, 22.0, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/drawer_angled.png"));
}

pub fn setup_bookshelf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();
    // far right wall, tall and thin
    // center_x ~93.2%, center_y ~59.2%, width ~13.6%, height ~73.1%
    let (x, y, w, h) = pct_to_world(93.2, 60., 30., 73.9, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/book_shelf_2.png"));
}

pub fn setup_table(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();
    // bottom edge flush with bottom wall: center_y = 100 - height/2 = 100 - 11.9 = 88.1%
    let (x, y, w, h) = pct_to_world(64., 88.1, 43.9, 23.8, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/table.png"));
}

pub fn setup_gift(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();
    // small square on top-left of drawer
    // center_x ~57%, center_y ~8%, width ~8%, height ~11%
    let (x, y, w, h) = pct_to_world(75.0, 20.0, 8.0, 11.0, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/gift.png"));
}


pub fn setup_heaters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();

    // iron heater: left edge flush with left wall (center_x = width/2 = 10.1%)
    // sits slightly above wall heater so they don't overlap
    let (x, y, w, h) = pct_to_world(10.1, 89.5, 20.2, 7.0, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/wall_heater.png"));

    // wall heater: flush with bottom wall (center_y = 100 - 3.75 = 96.25%)
    // shifted right enough to not overlap iron heater (right edge of iron = 20.2%, left edge of wall = 20.7%)
    let (x, y, w, h) = pct_to_world(31.0, 96.25, 20.6, 7.5, rw, rh);
    spawn_solid(&mut commands, &asset_server, x, y, w, h, 1., String::from("main/radiator.png"));
}

pub fn setup_wall_floor_boundary(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();

    // Sits at the visual boundary between the pink wall and the floor rug (~20% from top)
    let y = rh / 2.0 - (0.18 * rh);

    commands.spawn((
        Transform::from_xyz(0.0, y, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(rw / 2.0, 2.0),
        Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Min },
        MainCleanup,
    ));
}

pub fn setup_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let rw = window.resolution.width();
    let rh = window.resolution.height();

    commands.spawn((
        Sprite {
            image: asset_server.load("main/backgroundd.png"),
            custom_size: Some(Vec2::new(rw, rh)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        MainCleanup,
    ));
}