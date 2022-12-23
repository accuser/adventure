use crate::components::*;
use bevy::prelude::*;

const DANCING: AnimationIndices = AnimationIndices {
    first: 16,
    last: 19,
};
const DOWN: AnimationIndices = AnimationIndices { first: 0, last: 3 };
const LEFT: AnimationIndices = AnimationIndices { first: 4, last: 7 };
const RIGHT: AnimationIndices = AnimationIndices {
    first: 12,
    last: 15,
};
const UP: AnimationIndices = AnimationIndices { first: 8, last: 11 };

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_animation_indices)
            .add_system(animate_sprite);
    }
}

fn update_animation_indices(
    mut query: Query<(&mut AnimationIndices, &PlayerDirection, &PlayerState), With<Player>>,
) {
    query
        .iter_mut()
        .for_each(|(mut indices, direction, state)| {
            *indices = match (*direction, *state) {
                (_, PlayerState::Dancing) => DANCING,
                (PlayerDirection::Down, _) => DOWN,
                (PlayerDirection::Left, _) => LEFT,
                (PlayerDirection::Right, _) => RIGHT,
                (PlayerDirection::Up, _) => UP,
            };
        });
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &PlayerState,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    query
        .iter_mut()
        .for_each(|(indices, mut timer, state, mut sprite)| {
            if timer.tick(time.delta()).just_finished() {
                sprite.index = match *state {
                    PlayerState::Idle => indices.first,
                    _ => (sprite.index + 1) % (indices.last - indices.first + 1) + indices.first,
                }
            }
        });
}
