use crate::components::PlayerDirection;
use bevy::prelude::*;

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_direction);
    }
}

fn update_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut PlayerDirection>) {
    query.iter_mut().for_each(|mut direction| {
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down, KeyCode::Numpad2]) {
            *direction = PlayerDirection::Down
        } else if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::Numpad4]) {
            *direction = PlayerDirection::Left
        } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::Numpad6]) {
            *direction = PlayerDirection::Right
        } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Numpad8]) {
            *direction = PlayerDirection::Up
        } else if keyboard_input.any_pressed([KeyCode::Z, KeyCode::Numpad0]) {
            *direction = PlayerDirection::Down
        }
    })
}
