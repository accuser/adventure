use crate::components::*;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_movement);
    }
}

fn movement_vector(direction: PlayerDirection, state: PlayerState) -> (f32, f32) {
    let speed = match state {
        PlayerState::Walking => 1.,
        PlayerState::Running => 3.,
        _ => 0.,
    };

    return match direction {
        PlayerDirection::Down => (0.0, -speed),
        PlayerDirection::Left => (-speed, 0.0),
        PlayerDirection::Right => (speed, 0.0),
        PlayerDirection::Up => (0.0, speed),
    };
}

pub fn sprite_movement(
    time: Res<Time>,
    mut query: Query<(&mut PlayerDirection, &mut PlayerState, &mut Transform), With<Player>>,
) {
    query
        .iter_mut()
        .for_each(|(direction, state, mut transform)| {
            let (x, y) = movement_vector(*direction, *state);

            transform.translation.x += x * 64.0 * 5.0 * time.delta_seconds();
            transform.translation.y += y * 64.0 * 5.0 * time.delta_seconds();
        });
}
