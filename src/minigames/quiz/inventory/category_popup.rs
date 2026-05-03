use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, inventory::{basic::InventoryItemType, cleanup::{CleanupInventory, CleanupCategory}, player_model::PlayerModel}};

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
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
                String::from("quiz_game/hats/green.png"),
            ];
        },
        InventoryItemType::Undershirt => {
            return vec![String::from("quiz_game/hats/green.png")];
        },
        InventoryItemType::Outershirt => {
            return vec![String::from("quiz_game/hats/green.png")];
        },
        InventoryItemType::Pants => {
            return vec![String::from("quiz_game/hats/green.png")];
        },
        InventoryItemType::Shoes => {
            return vec![String::from("quiz_game/hats/green.png")];
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
    ))
    .with_children(|parent| {
        for path in items.iter() {
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
                    color: Color::BLACK,
                },
                ImageNode::new(asset_server.load(path)),
                CategoryPopupItem { path: path.clone(), selected: false },
                CleanupCategory
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