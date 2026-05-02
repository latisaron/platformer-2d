use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct InventoryPosition {
    value: usize,
}