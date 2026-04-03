use bevy::{prelude::*};
use std::collections::HashMap;

#[derive(Component)]
pub struct Level<T> {
    value: usize,
    config: HashMap<String, T>,
}