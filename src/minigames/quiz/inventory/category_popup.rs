use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, cleanup::CleanupQuiz, inventory::{basic::InventoryItemType, cleanup::{CleanupCategory, CleanupInventory}, player_model::{ClothingItem, PlayerModel}}};

#[derive(Component)]
pub struct CategoryPopup {
    iitype: InventoryItemType,
}

#[derive(Component)]
pub struct CategoryPopupItem {
    path: String,
    selected: bool,
}

fn map_items(iitype: &InventoryItemType) -> Vec<String> {
    match *iitype {
        InventoryItemType::Hat => {
            return vec![
                String::from("quiz_game/hats/0.png"),
                String::from("quiz_game/hats/1.png"),
                String::from("quiz_game/hats/2.png"),
                String::from("quiz_game/hats/3.png"),
                String::from("quiz_game/hats/4.png"),
                String::from("quiz_game/hats/5.png"),
                String::from("quiz_game/hats/6.png"),
                String::from("quiz_game/hats/7.png"),
            ];
        },
        InventoryItemType::Undershirt => {
            return vec![
                String::from("quiz_game/undershirts/0.png"),
                String::from("quiz_game/undershirts/2.png"),
                String::from("quiz_game/undershirts/3.png"),
                String::from("quiz_game/undershirts/4.png"),
                String::from("quiz_game/undershirts/5.png"),
                String::from("quiz_game/undershirts/6.png"),
                String::from("quiz_game/undershirts/7.png"),
                String::from("quiz_game/undershirts/8.png"),
                String::from("quiz_game/undershirts/9.png"),
                String::from("quiz_game/undershirts/10.png"),
                String::from("quiz_game/undershirts/11.png"),
                String::from("quiz_game/undershirts/12.png"),
                String::from("quiz_game/undershirts/13.png"),
                String::from("quiz_game/undershirts/14.png"),
                String::from("quiz_game/undershirts/15.png"),
                String::from("quiz_game/undershirts/16.png"),
            ];
        },
        InventoryItemType::Outershirt => {
            return vec![
                String::from("quiz_game/outershirts/0.png"),
                String::from("quiz_game/outershirts/1.png"),
                String::from("quiz_game/outershirts/2.png"),
                String::from("quiz_game/outershirts/3.png"),
                String::from("quiz_game/outershirts/4.png"),
                String::from("quiz_game/outershirts/5.png"),
                String::from("quiz_game/outershirts/6.png"),
                String::from("quiz_game/outershirts/7.png"),
                String::from("quiz_game/outershirts/8.png"),
                String::from("quiz_game/outershirts/9.png"),
                String::from("quiz_game/outershirts/10.png"),
                String::from("quiz_game/outershirts/11.png"),
                String::from("quiz_game/outershirts/12.png"),
                String::from("quiz_game/outershirts/13.png"),
                String::from("quiz_game/outershirts/14.png"),
            ];
        },
        InventoryItemType::Pants => {
            return vec![
                String::from("quiz_game/pants/0.png"),
                String::from("quiz_game/pants/1.png"),
                String::from("quiz_game/pants/2.png"),
                String::from("quiz_game/pants/3.png"),
                String::from("quiz_game/pants/4.png"),
                String::from("quiz_game/pants/5.png"),
                String::from("quiz_game/pants/6.png"),
                String::from("quiz_game/pants/7.png"),
                String::from("quiz_game/pants/8.png"),
                String::from("quiz_game/pants/9.png"),
                String::from("quiz_game/pants/10.png"),
                String::from("quiz_game/pants/11.png"),
                String::from("quiz_game/pants/12.png"),
            ];
        },
        InventoryItemType::Shoes => {
            return vec![
                String::from("quiz_game/shoes/0.png"),
                String::from("quiz_game/shoes/1.png"),
                String::from("quiz_game/shoes/2.png"),
                String::from("quiz_game/shoes/5.png"),
                String::from("quiz_game/shoes/6.png"),
                String::from("quiz_game/shoes/7.png"),
                String::from("quiz_game/shoes/8.png"),
                String::from("quiz_game/shoes/9.png"),
                String::from("quiz_game/shoes/10.png"),
            ];
        },
        _ => {
            return vec![];
        },
    }
}

pub fn setup_category_popup(
    window: &Single<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    category_type: &InventoryItemType,
    player_model: &Single<&PlayerModel>,
) {
    let max_width = window.width();
    let max_height = window.height();

    let popup_width = max_width * 0.8;
    let popup_height = max_height * 0.8;

    let left = max_width * 0.1;
    let top = max_height * 0.1;

    let items = map_items(category_type);

    commands.spawn((
        Node {
            width: Val::Px(popup_width),
            height: Val::Px(popup_height),

                padding: UiRect {
                    top: Val::Px(150.),
                    left: Val::Px(50.),
                    right: Val::Px(10.),
                    bottom: Val::Px(0.),
                },

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,

            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.),

            position_type: PositionType::Absolute,
            left: Val::Px(left),
            top: Val::Px(top),

            ..default()
        },
        ImageNode::new(asset_server.load("quiz_game/category_popup.png")),
        CategoryPopup { iitype: category_type.clone() },
        CleanupCategory,
        ZIndex(20),
        CleanupQuiz,
    ))
    .with_children(|parent| {
        for path in items.iter() {
            let selected = player_model.is_selected(category_type.clone(), path.clone());
            let outline_color = if selected { Color::srgb(1.0, 1.0, 0.0) } else { Color::BLACK };

            parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    ..default()
                },
                BackgroundColor(Color::srgba(0., 0., 0., 0.)),
                Outline {
                    width: Val::Px(2.),
                    offset: Val::Px(0.),
                    color: outline_color,
                },
                ImageNode::new(asset_server.load(path)),
                ZIndex(21),
                CategoryPopupItem { path: path.clone(), selected },
                CleanupCategory,
            ));
        }
    });

}

pub fn handle_outside_popup_click(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<MouseButton>>,
    popup: Single<Entity, With<CategoryPopup>>,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
) {
    let max_width = window.width();
    let max_height = window.height();

    let popup_width = max_width * 0.8;
    let popup_height = max_height * 0.8;

    let lwr_x = -popup_width / 2.;
    let upr_x = popup_width / 2.;
    let lwr_y = - popup_height / 2.;
    let upr_y = popup_height / 2.;


    if keys.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let x =  position[0] - window.width() / 2.0;
            let y = -(position[1] - window.height() / 2.0);

            if x < lwr_x || x > upr_x || y < lwr_y || y > upr_y {
                commands.entity(popup.entity()).despawn();
                quiz_game_state.set(QuizGameState::Choosing);
            }
        }
    }
}

pub fn handle_item_selection(
    // changing player clothing
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
    clothing_items_query: Query<(Entity, &ClothingItem)>,
    // actual category logic
    category: Single<&CategoryPopup>,
    mut params: ParamSet<(
        Query<(Entity, &Interaction, &CategoryPopupItem), (Changed<Interaction>, With<Button>)>,
        Query<(Entity, &mut CategoryPopupItem, &mut Outline), With<Button>>,
    )>,
    mut player_model: Single<&mut PlayerModel>,
) {
    // Detect which entity was pressed
    let mut pressed: Option<(Entity, String)> = None;
    for (entity, interaction, item) in params.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed = Some((entity, item.path.clone()));
            break;
        }
    }

    if let Some((pressed_entity, ref path)) = pressed {
        for (entity, mut item, mut outline) in params.p1().iter_mut() {
            if entity == pressed_entity {
                match category.iitype {
                    InventoryItemType::Hat => player_model.change_hat(&mut commands, &asset_server, &window, &clothing_items_query, item.path.clone()),
                    InventoryItemType::Undershirt => player_model.change_undershirt(&mut commands, &asset_server, &window, &clothing_items_query, item.path.clone()),
                    InventoryItemType::Outershirt => player_model.change_outershirt(&mut commands, &asset_server, &window, &clothing_items_query, item.path.clone()),
                    InventoryItemType::Pants => player_model.change_pants(&mut commands, &asset_server, &window, &clothing_items_query, item.path.clone()),
                    InventoryItemType::Shoes => player_model.change_shoes(&mut commands, &asset_server, &window, &clothing_items_query, item.path.clone()),
                    _ => {}
                }

                item.selected = true;
                outline.color = Color::srgb(1.0, 1.0, 0.0);
            } else {
                item.selected = false;
                outline.color = Color::BLACK;
            }
        }

        match category.iitype {
            InventoryItemType::Hat => player_model.hat = Some(path.clone()),
            InventoryItemType::Undershirt => player_model.undershirt = Some(path.clone()),
            InventoryItemType::Outershirt => player_model.outershirt = Some(path.clone()),
            InventoryItemType::Pants => player_model.pants = Some(path.clone()),
            InventoryItemType::Shoes => player_model.shoes = Some(path.clone()),
            _ => {}
        }
    }
}