use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Copy)]
pub enum PlayerDirection {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Component, Clone, Copy)]
pub enum PlayerState {
    Idle,
    Dancing,
    Walking,
}

#[derive(Component)]
pub struct Player;
