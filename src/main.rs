use bevy::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Adventure".to_string(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_startup_system(init)
        .add_plugin(PlayerPlugin)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
