use bevy::{math::VectorSpace, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, (animate_sprite, (integrate_phisics, appy_gravity, handle_player_movement).chain()));
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

#[derive(Component, Deref, DerefMut)]
struct Aceleration(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 78, y: 58 }, 11, 1, None, None);
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
        Transform::from_scale(Vec3::splat(1.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Velocity(Vec2::ZERO),
        Aceleration(Vec2::ZERO),
    ));
}

fn handle_player_movement(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<(&mut Aceleration, &mut Velocity), With<Player>>) {
    for (mut aceleration, mut velocity) in &mut query {
        **aceleration = Vec2::ZERO;
        if input.pressed(KeyCode::Space) {
            aceleration.y += 9.8 * 2.0;
        }

        if input.pressed(KeyCode::ArrowRight) {
            aceleration.x += 100.0;
        }
        
        if input.pressed(KeyCode::ArrowLeft) {
            aceleration.x -= 100.0;
        }
    
        **velocity += **aceleration * time.delta_secs();
    }
}

fn integrate_phisics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

fn appy_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in &mut query {
        velocity.y -= 9.8 * time.delta_secs();
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}