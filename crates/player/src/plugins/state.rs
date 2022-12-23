use crate::components::*;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_player_state);
    }
}

fn update_player_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerState, With<Player>>,
) {
    query.iter_mut().for_each(|mut state| {
        if keyboard_input.any_pressed([
            KeyCode::A,
            KeyCode::D,
            KeyCode::S,
            KeyCode::W,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Numpad2,
            KeyCode::Numpad4,
            KeyCode::Numpad6,
            KeyCode::Numpad8,
        ]) {
            *state = if keyboard_input.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
                PlayerState::Running
            } else {
                PlayerState::Walking
            }
        } else if keyboard_input.any_pressed([KeyCode::Z, KeyCode::Numpad0]) {
            *state = PlayerState::Dancing
        } else {
            *state = PlayerState::Idle
        }
    })
}
