mod components;
mod plugins;

use bevy::prelude::*;
use components::*;
use plugins::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init).add_plugins(Plugins);
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 20, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::new(5., 5., 1.)),
            ..default()
        },
        PlayerDirection::Down,
        PlayerState::Idle,
        AnimationIndices { first: 0, last: 0 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
