use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::minigames::MinigameState;

const INTERACT_RADIUS: f32 = 500.0;

#[derive(Component)]
pub enum Interactable {
    Knife,
    Shoot,
    Quiz,
}

pub fn check_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    player_query: Query<&Transform, With<Velocity>>,
    interactable_query: Query<(&Transform, &Interactable)>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_transform) = player_query.single() else { 
        return 
    };
    let player_pos = player_transform.translation.truncate();

    for (obj_transform, interactable) in &interactable_query {
        let obj_pos = obj_transform.translation.truncate();
        let dist = player_pos.distance(obj_pos);
        if dist <= INTERACT_RADIUS {
                        match interactable {
                Interactable::Knife => next_state.set(MinigameState::Knife),
                Interactable::Shoot => next_state.set(MinigameState::Shoot),
                Interactable::Quiz  => next_state.set(MinigameState::Quiz),
            }
            return;
        }
    }
}