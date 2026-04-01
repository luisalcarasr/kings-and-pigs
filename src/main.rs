use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(
        asset_server.load("kings-and-pigs.png"),
    ));
}
