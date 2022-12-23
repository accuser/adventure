mod animation;
mod direction;
mod movement;
mod state;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use animation::AnimationPlugin;
use direction::DirectionPlugin;
use movement::MovementPlugin;
use state::StatePlugin;

pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AnimationPlugin)
            .add(DirectionPlugin)
            .add(MovementPlugin)
            .add(StatePlugin)
    }
}
