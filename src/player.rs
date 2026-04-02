use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, animate_sprite);
        app.add_systems(Update, handle_player_movement);
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

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
        Transform::from_scale(Vec3::splat(10.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

fn handle_player_movement(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    for mut transform in &mut query {
        transform.translation += direction * time.delta_secs() * 100.0;
    }

}
