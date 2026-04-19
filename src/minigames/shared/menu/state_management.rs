use bevy::{prelude::*};
use super::menu_item_type::{MenuItemType};

#[derive(Component)]
pub struct CleanupMenu;

#[derive(Component)]
pub struct Menu {
    pub position: usize,
    pub number_items: usize,
}

#[derive(Component)]
pub struct MenuItem {
    pub position: usize,
    pub menu_item_type: MenuItemType,
}


// this state is used to mention if the game is on-going or we're in the menu
#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    Menu,
    Play,
}

// this state is to define what kind of menu it is
#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuType {
    None, // not really in a menu
    MainEscape, // this will be used as the default menu when hitting Escape in the main game
    MinigameEscape, // this will be used as the default menu when hitting Escape in a minigame
    Win, // this will be used in the case of winning the level
    Lost, // this will be used in the case of losing the level
}




pub fn cleanup_menu(
    mut commands: Commands,
    cleanup_items: Query<Entity, With<CleanupMenu>>,
) {
    for item in cleanup_items {
        commands.entity(item).despawn();
    }
}

pub fn setup_menu(
    mut commands: Commands,
    window: Single<& Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    items: Vec<MenuItemType>,
    text: String,
    number_of_items: usize,
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
    ));

    commands.spawn((
        Menu {
            position: 0,
            number_items: number_of_items,
        },
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
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
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(10),
                    ..default()
                },
            )).with_children(|parent| {

                parent.spawn((
                        Node {
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::default(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::new(text),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ));
                        });
                    });

                for (i, label) in items.iter().enumerate() {
                    let color =
                    if i == 0 {
                        Color::srgb(0.3, 0.3, 0.8) // highlighted
                    } else {
                        Color::NONE
                    };
                    parent.spawn((
                        Node {
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(color),
                        MenuItem {
                            position: i,
                            menu_item_type: label.clone(),
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::default(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::new(label.text()),
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
