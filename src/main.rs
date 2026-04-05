mod player;
mod scenario;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledPhysicsPlugin, TiledPhysicsRapierBackend};
use bevy_ecs_tiled::tiled::TiledPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::player::PlayerPlugin;
use crate::scenario::ScenarioPlugin;

fn main() {
    App::new()
        /*
        * `ImagePlugin::default_nearest()` is used to prevent blurring 
        * of pixel art textures when they are scaled up.
        */
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .add_plugins(TiledPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScenarioPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
