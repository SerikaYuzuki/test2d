//! Renders a 2D scene containing a single, moving sprite.

use animation::PlayerAnimationPlugin;
use bevy::{prelude::*, ecs::query};

pub mod animation;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .add_plugins(PlayerAnimationPlugin)
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
    
}
