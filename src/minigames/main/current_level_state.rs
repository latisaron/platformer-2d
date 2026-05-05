use bevy::{prelude::*};

#[derive(Component)]
pub struct KnifeLevel {
    pub val: usize,
}

#[derive(Component)]
pub struct GunLevel {
    pub val: usize,
}

#[derive(Component)]
pub struct QuizLevel {
    pub val: usize,
}

pub fn setup_gun_level(
    mut commands: Commands,
) {
    commands.spawn(
        GunLevel{ val: 1},
    );
}

pub fn setup_knife_level(
    mut commands: Commands,
) {
    commands.spawn(
        KnifeLevel{ val: 1},
    );
}

pub fn setup_quiz_level(
    mut commands: Commands,
) {
    commands.spawn(
        QuizLevel{ val: 1},
    );
}