use bevy::{prelude::*, window::PrimaryWindow};

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

fn inventory_matching(string: &str) -> InventoryItemType {
    match string {
        "quiz_game/inventory_start.png" => InventoryItemType::Header,
        "quiz_game/inventory_end.png" => InventoryItemType::Footer,
        "quiz_game/hat_category.png" => InventoryItemType::Hat,
        "quiz_game/undershirt_category.png" => InventoryItemType::Undershirt,
        "quiz_game/outershirt_category.png" => InventoryItemType::Outershirt,
        "quiz_game/pants_category.png" => InventoryItemType::Pants,
        "quiz_game/shoes_category.png" => InventoryItemType::Shoes,
        "quiz_game/empty.png" => InventoryItemType::Empty,
        _ => InventoryItemType::Empty
    }
}

#[derive(Component)]
pub struct InventoryItem {
    iitype: InventoryItemType,
}
fn setup_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            width: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        let mut spawn_item = |path: &'static str| {
            let correct_item_type = inventory_matching(path);
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                ImageNode::new(asset_server.load(path)),
                InventoryItem { iitype: correct_item_type },
            ));
        };

        spawn_item(INVENTORY_HEADER);
        spawn_item(HAT_CATEGORY);
        spawn_item(UNDERSHIRT_CATEGORY);
        spawn_item(OUTERSHIRT_CATEGORY);
        spawn_item(PANTS_CATEGORY);
        spawn_item(SHOES_CATEGORY);
        spawn_item(EMPTY);
        spawn_item(INVENTORY_FOOTER);
    });
}