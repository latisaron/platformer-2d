use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{QuizGameState, cleanup::CleanupQuiz, inventory::{category_popup::setup_category_popup, cleanup::CleanupInventory, player_model::PlayerModel}};

pub const INVENTORY_HEADER: &'static str = "quiz_game/inventory_start.png";
pub const INVENTORY_FOOTER: &'static str = "quiz_game/inventory_end.png";
pub const HAT_CATEGORY: &'static str = "quiz_game/hat_category.png";
pub const UNDERSHIRT_CATEGORY: &'static str = "quiz_game/undershirt_category.png";
pub const OUTERSHIRT_CATEGORY: &'static str = "quiz_game/outershirt_category.png";
pub const PANTS_CATEGORY: &'static str = "quiz_game/pants_category.png";
pub const SHOES_CATEGORY: &'static str = "quiz_game/shoes_category.png";
pub const EMPTY: &'static str = "quiz_game/empty.png";
pub const DOWN_ARROW: &'static str = "quiz_game/down_arrow.png";
pub const RIGHT_ARROW: &'static str = "quiz_game/right_arrow.png";

pub const INVENTORY_ITEM_Z_INDEX: f32 = 2.;
pub const INVENTORY_ARROW_Z_INDEX: f32 = 3.;


#[derive(Clone, Eq, PartialEq)]
pub enum InventoryItemType {
    Header,
    Footer,
    Hat,
    Undershirt,
    Outershirt,
    Pants,
    Shoes,
    Empty,
}

#[derive(Component)]
pub struct InventoryItem {
    iitype: InventoryItemType,
    open: bool,
}

pub fn setup_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            width: Val::Px(430.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,

            position_type: PositionType::Absolute,
            left: Val::Px(1500.0),
            top: Val::Px(0.0),
            ..default()
        },
        CleanupInventory,
        CleanupQuiz,
    ))
    .with_children(|parent| {
        let items = [
            (INVENTORY_HEADER, InventoryItemType::Header),
            (HAT_CATEGORY, InventoryItemType::Hat),
            (UNDERSHIRT_CATEGORY, InventoryItemType::Undershirt),
            (OUTERSHIRT_CATEGORY, InventoryItemType::Outershirt),
            (PANTS_CATEGORY, InventoryItemType::Pants),
            (SHOES_CATEGORY, InventoryItemType::Shoes),
            (EMPTY, InventoryItemType::Empty),
            (INVENTORY_FOOTER, InventoryItemType::Footer),
        ];

        for (path, item_type) in items.iter() {
            let correct_height =
                if *item_type == InventoryItemType::Empty {
                    380.
                } else {
                    100.
                };

            let is_clickable = matches!(
                item_type,
                InventoryItemType::Hat
                    | InventoryItemType::Undershirt
                    | InventoryItemType::Outershirt
                    | InventoryItemType::Pants
                    | InventoryItemType::Shoes
            );

            let mut entity = parent.spawn((
                Node {
                    width: Val::Px(430.),
                    height: Val::Px(correct_height),
                    ..default()
                },
                ImageNode::new(asset_server.load(*path)),
                InventoryItem { iitype: item_type.clone(), open: false },
                CleanupInventory,
                CleanupQuiz,
            ));

            if is_clickable {
                entity.insert(Button);
            }            
        }
    });
}

pub fn handle_inventory_clicks(
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<
        (&Interaction, &mut InventoryItem),
        (Changed<Interaction>, With<Button>)
    >,
    mut quiz_game_state: ResMut<NextState<QuizGameState>>,
    player_model: Single<&PlayerModel>,
) {
    for (interaction, mut item) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match item.iitype {
                InventoryItemType::Hat => {
                    quiz_game_state.set(QuizGameState::Browsing);
                    setup_category_popup(&window, &mut commands, &asset_server, &item.iitype, &player_model);
                }
                InventoryItemType::Undershirt => {
                    quiz_game_state.set(QuizGameState::Browsing);
                    setup_category_popup(&window, &mut commands, &asset_server, &item.iitype, &player_model);
                }
                InventoryItemType::Outershirt => {
                    quiz_game_state.set(QuizGameState::Browsing);
                    setup_category_popup(&window, &mut commands, &asset_server, &item.iitype, &player_model);
                }
                InventoryItemType::Pants => {
                    quiz_game_state.set(QuizGameState::Browsing);
                    setup_category_popup(&window, &mut commands, &asset_server, &item.iitype, &player_model);
                }
                InventoryItemType::Shoes => {
                    quiz_game_state.set(QuizGameState::Browsing);
                    setup_category_popup(&window, &mut commands, &asset_server, &item.iitype, &player_model);
                }
                _ => {}
            }

            // example: toggle state
            item.open = !item.open;
        }
    }
}