use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, GravityScale, RigidBody, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, (animate_sprite, (handle_player_movement).chain()));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("characters/playables/king-human/idle.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: 37,
            y: 30,
        },
        11,
        1,
        Some(UVec2 { x: 41, y: 0 }),
        Some(UVec2 { x: 9, y: 14 }),
    );
    let textures_atlas_layout = textures_atlas_layout.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 10 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: textures_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(2.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(37.0 / 2.0, 28.0 / 2.0),
        Velocity::zero(),
        GravityScale(1.0),
    ));
}

fn handle_player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        if input.pressed(KeyCode::Space) {
            velocity.linvel.y = 300.0;
        }

        if input.pressed(KeyCode::ArrowRight) {
            velocity.linvel.x = 150.0;
        } else if input.pressed(KeyCode::ArrowLeft) {
            velocity.linvel.x = -150.0;
        } else {
            velocity.linvel.x = 0.0;
        }
    }
}
