//! Renders a 2D scene containing a single, moving sprite.

use animation::{HelloPlugin, AnimationIndices, AnimationTimer};
use bevy::{prelude::*, ecs::query};

pub mod animation;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .add_plugins(HelloPlugin)
        .add_systems(Update, play_player_animation)
        .run();
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
pub enum PlayerState{
    Idle,
    Run,
}

/////////////////////////////////////////////
/// This is tag component for player
/////////////////////////////////////////////
#[derive(Component)]
pub struct Player;

fn add_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let entity_id = 
        commands.spawn(
            (
                Name("Player".to_string()), 
                Position{x: 0.0, y: 0.0}, Velocity{dx: 0.0, dy: 0.0}, 
                Player,
                animation::load_idle_image(asset_server, texture_atlases),
                PlayerState::Idle,
            ),   
        ).id();

    commands.spawn(Camera2dBundle::default());
}

fn play_player_animation(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    mut query2: Query<(&Player,
        &PlayerState)>,
){
    animation::animate_sprite(time, query, query2);
}
