use bevy::{prelude::*, window::PrimaryWindow};

use crate::minigames::quiz::{cleanup::CleanupQuiz, inventory::{
    HAT_Z_INDEX, OUTERSHIRT_Z_INDEX, PANTS_Z_INDEX, PLAYER_MODEL_Z_INDEX, SHOES_Z_INDEX, UNDERSHIRT_Z_INDEX, WHITE_FRAME_Z_INDEX, basic::InventoryItemType}};

const IMAGE_WIDTH_PERCENTAGE: f32 = 0.3;
const IMAGE_HEIGH_PERCENTAGE: f32 = 0.8;

#[derive(Component)]
pub struct ClothingItem {
    iitype: InventoryItemType,
}

#[derive(Component)]
pub struct PlayerModel {
    pub hat: Option<String>,
    pub undershirt: Option<String>,
    pub outershirt: Option<String>,
    pub pants: Option<String>,
    pub shoes: Option<String>,
}

impl PlayerModel {
    pub fn cleanup_existing_item(
        &mut self,
        commands: &mut Commands,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        item_type: InventoryItemType,
    ) {
        for item in clothing_items_query {
            if item.1.iitype == item_type {
                commands.entity(item.0).despawn();
            }
        }
    }

    pub fn is_selected(&self, iitype: InventoryItemType, path: String) -> bool {
        match iitype {
            InventoryItemType::Hat => { 
                if let Some(cp) = &self.hat {
                    path == *cp
                } else {
                    false
                }
             }
            InventoryItemType::Undershirt => {
                if let Some(cp) = &self.undershirt {
                    path == *cp
                } else {
                    false
                }
            }
            InventoryItemType::Outershirt => {
                if let Some(cp) = &self.outershirt {
                    path == *cp
                } else {
                    false
                }
            }
            InventoryItemType::Pants => {
                if let Some(cp) = &self.pants {
                    path == *cp
                } else {
                    false
                }
            }
            InventoryItemType::Shoes => {
                if let Some(cp) = &self.shoes {
                    path == *cp
                } else {
                    false
                }
            }
            _ => false
        }
    }

    pub fn change_hat(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window: &Single<&Window, With<PrimaryWindow>>,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        path: String,
    ) {
        self.cleanup_existing_item(commands, clothing_items_query, InventoryItemType::Hat);
        self.hat = Some(path.clone());
        let window_width = window.width();
        let window_height = window.height();

        let image_height = IMAGE_HEIGH_PERCENTAGE * window_height;
        
        let item_height = 0.3707 * image_height;
        let item_y: f32 = -(0.187 * image_height - image_height / 2.);

        commands.spawn((
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, item_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., item_y, HAT_Z_INDEX),
            WhiteBackground,
            ClothingItem { iitype: InventoryItemType::Hat },
            CleanupQuiz,
        ));
    }

    pub fn change_undershirt(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window: &Single<&Window, With<PrimaryWindow>>,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        path: String,
    ) {
        self.cleanup_existing_item(commands, clothing_items_query, InventoryItemType::Undershirt);
        self.undershirt = Some(path.clone());
        let window_width = window.width();
        let window_height = window.height();

        let image_height = IMAGE_HEIGH_PERCENTAGE * window_height;
        
        let item_height = 0.2721 * image_height;
        let item_y: f32 = -(0.5068 * image_height - image_height / 2.);

        commands.spawn((
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, item_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., item_y, UNDERSHIRT_Z_INDEX),
            WhiteBackground,
            ClothingItem { iitype: InventoryItemType::Undershirt },
            CleanupQuiz,
        ));
    }

    pub fn change_outershirt(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window: &Single<&Window, With<PrimaryWindow>>,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        path: String,
    ) {
        self.cleanup_existing_item(commands, clothing_items_query, InventoryItemType::Outershirt);
        self.outershirt = Some(path.clone());
        let window_width = window.width();
        let window_height = window.height();

        let image_height = IMAGE_HEIGH_PERCENTAGE * window_height;
        
        let item_height = 0.2721 * image_height;
        let item_y: f32 = -(0.5068 * image_height - image_height / 2.);

        commands.spawn((
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, item_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., item_y, OUTERSHIRT_Z_INDEX),
            WhiteBackground,
            ClothingItem { iitype: InventoryItemType::Outershirt },
            CleanupQuiz,
        ));
    }

    pub fn change_pants(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window: &Single<&Window, With<PrimaryWindow>>,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        path: String,
    ) {
        self.cleanup_existing_item(commands, clothing_items_query, InventoryItemType::Pants);
        self.pants = Some(path.clone());
        let window_width = window.width();
        let window_height = window.height();

        let image_height = IMAGE_HEIGH_PERCENTAGE * window_height;
        
        let item_height = 0.3571 * image_height;
        let item_y: f32 = -(0.8214 * image_height - image_height / 2.);

        commands.spawn((
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, item_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., item_y, PANTS_Z_INDEX),
            WhiteBackground,
            ClothingItem { iitype: InventoryItemType::Pants },
            CleanupQuiz,
        ));
    }

    pub fn change_shoes(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window: &Single<&Window, With<PrimaryWindow>>,
        clothing_items_query: &Query<(Entity, &ClothingItem)>,
        path: String,
    ) {
        self.cleanup_existing_item(commands, clothing_items_query, InventoryItemType::Shoes);
        self.shoes = Some(path.clone());
        let window_width = window.width();
        let window_height = window.height();

        let image_height = IMAGE_HEIGH_PERCENTAGE * window_height;
        
        let item_height = 0.1496 * image_height;
        let item_y: f32 = -(0.9251 * image_height - image_height / 2.);

        commands.spawn((
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::new(IMAGE_WIDTH_PERCENTAGE * window_width, item_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform::from_xyz(0., item_y, SHOES_Z_INDEX),
            WhiteBackground,
            ClothingItem { iitype: InventoryItemType::Shoes },
            CleanupQuiz,
        ));
    }
}

#[derive(Component)]
pub struct WhiteBackground;

pub fn spawn_player_model(mut commands: Commands) {
    commands.spawn((PlayerModel {
            hat: None,
            undershirt: None,
            outershirt: None,
            pants: None,
            shoes: None,
        },
        CleanupQuiz,
    ));

}

pub fn setup_player_model(
    mut commands: Commands,
    mut player_model: Single<&mut PlayerModel>,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
    clothing_items_query: Query<(Entity, &ClothingItem)>,
) {
    player_model.change_hat(&mut commands, &asset_server, &window, &clothing_items_query, String::from("quiz_game/hats/0.png"));
    player_model.change_undershirt(&mut commands, &asset_server, &window, &clothing_items_query,  String::from("quiz_game/undershirts/0.png"));
    player_model.change_pants(&mut commands, &asset_server, &window, &clothing_items_query,  String::from("quiz_game/pants/0.png"));
    player_model.change_shoes(&mut commands, &asset_server, &window, &clothing_items_query,  String::from("quiz_game/shoes/0.png"));
}

// raw_man
// width = 127
// height = 294

// head = 1, 109 => height = 37.07, center = 18.70
// torso = 109, 189 => height = 27.21, center = 50.68
// legs = 189, 294 => height = 35.71, center = 82.14
// feet = 250, 294 => height = 14.96, center = 92.51

