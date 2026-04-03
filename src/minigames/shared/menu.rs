use std::process::exit;

use bevy::{prelude::*};

#[derive(Component)]
pub struct CleanupMenu;

#[derive(Component)]
pub struct MenuPosition(i32);

#[derive(Component)]
pub struct MenuItem(pub i32);

#[derive(Component)]
pub struct MenuSelected;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    Menu,
    Play,
}

pub fn listen_keystroke_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Single<&mut MenuPosition>,
    mut menu_query: Query<(&mut Visibility), With<CleanupMenu>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        query.0 = 0;
        for mut menu_item in &mut menu_query {
            *menu_item = Visibility::Visible;
        }
    }
}

pub fn listen_keystroke_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Single<&mut MenuPosition>,
    mut menu_items: Query<(&MenuItem, &mut BackgroundColor), With<MenuItem>>,
    mut menu_query: Query<(&mut Visibility), With<CleanupMenu>>,
) {
     if keys.just_pressed(KeyCode::ArrowUp) {
        if query.0 == 0 {
            query.0 = 2;
        } else {
            query.0 -= 1;
        }
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        if query.0 == 2{
            query.0 = 0;
        } else {
            query.0 += 1;
        }
    } else if keys.just_pressed(KeyCode::Enter) {
        if query.0 == 0 {
            game_state.set(GameState::Play);
            for mut menu_item in &mut menu_query {
                *menu_item = Visibility::Hidden;
            }
        } else if query.0 == 1 {
            // also restart somehow
            game_state.set(GameState::Play);
            for mut menu_item in &mut menu_query {
                *menu_item = Visibility::Hidden;
            }
        } else if query.0 == 2 {
            exit(0);
        }
    }

    for (item, mut color) in menu_items.iter_mut() {
        if item.0 as i32 == query.0 {
            *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.8)); // highlighted
        } else {
            *color = BackgroundColor(Color::NONE);
        }
    }
}

pub fn setup_menu(
    mut commands: Commands,
    window: Single<& Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();
    
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            window_width,
            window_height,
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        Transform::from_xyz(0., 0.0, 100.),
        CleanupMenu,
        Visibility::Hidden,
    ));

    commands.spawn((
        MenuPosition(0),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        CleanupMenu,
    )).with_children(|parent| {

        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(10),
                ..default()
            },
        )).with_children(|parent| {

            let items = ["Start", "Restart", "Exit"];

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(10),
                    ..default()
                },
            )).with_children(|parent| {

                for (i, label) in items.iter().enumerate() {
                    parent.spawn((
                        Node {
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        MenuItem(i as i32),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::default(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::new(*label),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ));
                        });
                    });
                }
            });

        });
    });
}
