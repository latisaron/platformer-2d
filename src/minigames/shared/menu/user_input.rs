use bevy::{prelude::*};
use crate::minigames::{
    shared::menu::{
        menu_item_type::MenuItemType,
        state_management::{
            GameState, Menu, MenuItem
        },
        menu_action::{MenuAction},
    }
};

pub fn listen_keystroke_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

pub fn listen_keystroke_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_action: ResMut<NextState<MenuAction>>,
    mut query: Single<&mut Menu>,
    mut menu_items: Query<(&MenuItem, &mut BackgroundColor), With<MenuItem>>,
) {
     if keys.just_pressed(KeyCode::ArrowUp) {
        if query.position == 0 {
            query.position = query.number_items - 1;
        } else {
            query.position -= 1;
        }

        // hsould move
        for (item, mut color) in menu_items.iter_mut() {
            if item.position == query.position {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.8)); // highlighted
            } else {
                *color = BackgroundColor(Color::NONE);
            }
        }
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        if query.position == query.number_items - 1 {
            query.position = 0;
        } else {
            query.position += 1;
        }

        // should move
        for (item, mut color) in menu_items.iter_mut() {
            if item.position == query.position {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.8)); // highlighted
            } else {
                *color = BackgroundColor(Color::NONE);
            }
        }
    } else if keys.just_pressed(KeyCode::Enter) {
        if let Some(result) =  menu_items.into_iter().find(|(item, _)| item.position == query.position) {
            match result.0.menu_item_type {
                MenuItemType::Continue => {
                    menu_action.set(MenuAction::PreContinue);
                },
                MenuItemType::Restart => {
                    menu_action.set(MenuAction::PreRestart);
                },
                MenuItemType::Exit => {
                    menu_action.set(MenuAction::PreExit);
                },
            }
        }
    }
}

