use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

#[derive(Component)]
struct CharacterAnimations {
    idle: AnimationIndices,
    run: AnimationIndices,
    attack: AnimationIndices,
    dash: AnimationIndices,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the texture!!
    let texture_handle = asset_server.load("tewi_material01.png");

    // Process the texture into an atlas!!
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 96.0), 10, 5, None, None);

    // Add the atlas to our asset storage using add function!!
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 11 };

    // Spawn Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a sprite with the default transform, and with the animation indices
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
    ));
}

