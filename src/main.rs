mod player;
use bevy::prelude::*;

use crate::player::PlayerPlugin;

fn main() {
    App::new()
        /*
        * `ImagePlugin::default_nearest()` is used to prevent blurring 
        * of pixel art textures when they are scaled up.
        */
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
