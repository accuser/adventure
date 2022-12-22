use crate::components::*;
use bevy::prelude::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut PlayerState,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    query
        .iter_mut()
        .for_each(|(indices, mut timer, state, mut sprite)| {
            timer.tick(time.delta());

            if timer.just_finished() {
                match *state {
                    PlayerState::Idle => sprite.index = indices.first,
                    _ => {
                        if sprite.index >= indices.first && sprite.index < indices.last {
                            sprite.index = sprite.index + 1;
                        } else {
                            sprite.index = indices.first;
                        }
                    }
                }
            }
        });
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut PlayerDirection,
            &mut PlayerState,
        ),
        With<Player>,
    >,
) {
    for (mut indices, mut direction, mut state) in &mut query {
        if keyboard_input.pressed(KeyCode::Z) {
            *direction = PlayerDirection::Down;
            *state = PlayerState::Dancing;
            *indices = AnimationIndices {
                first: 16,
                last: 19,
            }
        } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            *direction = PlayerDirection::Down;
            *state = PlayerState::Walking;
            *indices = AnimationIndices { first: 0, last: 3 }
        } else if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            *direction = PlayerDirection::Left;
            *state = PlayerState::Walking;
            *indices = AnimationIndices { first: 4, last: 7 }
        } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            *direction = PlayerDirection::Right;
            *state = PlayerState::Walking;
            *indices = AnimationIndices {
                first: 12,
                last: 15,
            }
        } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            *direction = PlayerDirection::Up;
            *state = PlayerState::Walking;
            *indices = AnimationIndices { first: 8, last: 11 }
        } else {
            *state = PlayerState::Idle;
        }
    }
}

pub fn sprite_movement(
    time: Res<Time>,
    mut query: Query<(&mut PlayerDirection, &mut PlayerState, &mut Transform), With<Player>>,
) {
    for (direction, state, mut transform) in &mut query {
        match (*direction, *state) {
            (PlayerDirection::Down, PlayerState::Walking) => {
                transform.translation.y -= 64.0 * 5.0 * time.delta_seconds()
            }
            (PlayerDirection::Left, PlayerState::Walking) => {
                transform.translation.x -= 64.0 * 5.0 * time.delta_seconds()
            }
            (PlayerDirection::Right, PlayerState::Walking) => {
                transform.translation.x += 64.0 * 5.0 * time.delta_seconds()
            }
            (PlayerDirection::Up, PlayerState::Walking) => {
                transform.translation.y += 64.0 * 5.0 * time.delta_seconds()
            }
            _ => {}
        }
    }
}
