use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Handle},
    camera::ClearColor,
    color::Color,
    ecs::system::{Commands, Res}, utils::default,
};
use bevy_ecs_tiled::prelude::{TiledFilter, TiledMap, TiledMapAsset, TiledPhysicsRapierBackend, TiledPhysicsSettings};
use bevy_ecs_tilemap::prelude::TilemapAnchor;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(
            63.0 / 255.0,
            56.0 / 255.0,
            81.0 / 255.0,
        )));
        app.add_systems(Startup, setup_scenario);
    }
}

fn setup_scenario(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map: Handle<TiledMapAsset> = asset_server.load("maps/level_1.tmx");
    commands.spawn((
        TiledMap(map),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
            tiles_layer_filter: TiledFilter::Names(vec!["collision".into()]),
            ..default()
        },
    ));
}
