use bevy::{prelude::*, ecs::query};
use crate::{Player, PlayerState};


/////////////////////////////////////////////
/// Setting up the plugin
/////////////////////////////////////////////
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, load_idle_image);
        // app.add_systems(Update,animate_sprite);

    }
}

/////////////////////////////////////////////
/// Component Section
/////////////////////////////////////////////
/// 
/// This component is container for animation counter 
#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}


/// Animation Timer Component
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        
    )>,
    mut query2: Query<(&Player,
        &PlayerState)>,
) {

    if query2.iter().len() == 0 {
        println!("No Player");
        return;
    } else {
        let (player, player_state) = &mut query2.get_single().unwrap();
        match player_state {
            PlayerState::Idle => {
                println!("Idle");
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
            },
            PlayerState::Run => {
                println!("Run");
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
            },
        }
    }
    // TODO Switch animation based on player or enemy state
    
}

fn animate (time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        
    )>,){
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
enum AnimationState {
    Idle,
    Run,
}

#[derive(Bundle)]
pub struct PlayerAnimation{
    spritesheetbundle: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    animation_state: AnimationState,
}


/// This component is List of animation counter
#[derive(Resource, Default)]
pub struct PlayerSpriteHandles {
    idle: HandleUntyped,
    run: HandleUntyped,
}

#[derive(Resource)]
struct IdleSpriteSheet {
    texture_atlas: Handle<TextureAtlas>,
}

pub fn load_idle_image(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> PlayerAnimation
{

    // TODO Load Different animation indices from files
    let texture_handle = asset_server.load("tewi_material01.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 96.0), 10, 5, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);



    let texture_handle = asset_server.load("tewi_material02.png");
    let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 96.0), 10, 5, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);



    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 11 };
    // TODO Set animation into different component rand set it into player entity
    let player_animation = PlayerAnimation{
        spritesheetbundle: SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_indices.first),
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },
    animation_indices: animation_indices,
    animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    animation_state: AnimationState::Idle,
    };
    player_animation
}
