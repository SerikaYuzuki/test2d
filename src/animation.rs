use bevy::{prelude::*, ecs::query, asset::LoadState};
use crate::{Player, PlayerState};


/////////////////////////////////////////////
/// Setting up the plugin
/////////////////////////////////////////////
pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlayerSpriteHandles>()
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), load_images)
        .add_systems(Update, check_if_image_loaded.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .add_systems(Update, animate_sprite);
    }
}

/// Component : Player Sprite Handles
/// Contains Vec<UntypedHandle>
#[derive(Resource, Default)]
struct PlayerSpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Component)]
struct PlayerTextureAtlasHandles {
    idle_texture_atlas_handle: Handle<TextureAtlas>,
    jump_texture_atlas_handle: Handle<TextureAtlas>,
}

/// Component : Player Sprite Index
/// Contains Number of Sprites and Indexes for each animation
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

/// Component : Player Sprite Index Container
/// Contains AnimationIndices for each animation
#[derive(Component)]
struct AnimationIndicesContainer {
    idle: AnimationIndices,
    walk: AnimationIndices,
}

/// Component : Player Sprite Animation Timer
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/// Component : AppState for Loading
/// Contains LoadState : Loading, Failed, Loaded
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

/// System : Load Player Sprites Images
fn load_images (
    mut player_sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
){
    player_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

/// System : Check if Player Sprites Images are loaded
fn check_if_image_loaded(
    asset_server: Res<AssetServer>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    mut state: ResMut<NextState<AppState>>,
){
    if let 
    LoadState::Loaded = asset_server.get_group_load_state(
        player_sprite_handles.handles.iter().map(|h| h.id())) 
        {
        state.set(AppState::Finished);
    }
}


/// System : Add Sprite Sheet Components to Player
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    mut textures: ResMut<Assets<Image>>,
){

    // Build Texure Atlas
    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    // Check All Images Loaded and Add to Texture Atlas
    for handle in &player_sprite_handles.handles {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };

        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    texture_atlases.add(texture_atlas);

    let idle_sheet_image_handle : Handle<Image> = player_sprite_handles.handles[0].clone().typed();
    let idle_texture_atlas =
        TextureAtlas::from_grid(idle_sheet_image_handle, Vec2::new(80.0, 96.0), 10, 5, None, None);

    let player_idle_animation_indices = AnimationIndices {
        first: 0,
        last: 11,
    };
    let player_walk_animation_indices = AnimationIndices {
        first: 0,
        last: 9,
    };
    let player_animation_indices_container = AnimationIndicesContainer {
        idle: player_idle_animation_indices,
        walk: player_walk_animation_indices,
    };


    let idle_texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(idle_texture_atlas);

    let walk_sheet_image_handle : Handle<Image> = player_sprite_handles.handles[1].clone().typed();

    let walk_texture_atlas =
        TextureAtlas::from_grid(walk_sheet_image_handle, Vec2::new(80.0, 96.0), 10, 2, None, None);

    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(walk_texture_atlas);

    let player_texture_atlas_handles = PlayerTextureAtlasHandles {
        idle_texture_atlas_handle: idle_texture_atlas_handle.clone(),
        jump_texture_atlas_handle: texture_atlas_handle.clone(),
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: idle_texture_atlas_handle,
            sprite: TextureAtlasSprite::new(player_animation_indices_container.idle.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        player_animation_indices_container,
        AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
        player_texture_atlas_handles,
    ));
}

// System : Update Animate Player Sprites
fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &AnimationIndicesContainer,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &PlayerTextureAtlasHandles,
        &mut Handle<TextureAtlas>
    )>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        for (entity, indices, mut timer, mut sprite, player_texture_atlas_handles, texture_atlas_handle) in &mut query {

            commands.entity(entity).remove::<Handle<TextureAtlas>>();
            commands.entity(entity).insert(player_texture_atlas_handles.idle_texture_atlas_handle.clone());

            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.idle.last {
                    indices.idle.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
    
    if keyboard_input.pressed(KeyCode::S) {
        for (entity, indices, mut timer, mut sprite, player_texture_atlas_handles, mut texture_atlas_handle) in &mut query {

            commands.entity(entity).remove::<Handle<TextureAtlas>>();
            commands.entity(entity).insert(player_texture_atlas_handles.jump_texture_atlas_handle.clone());
            
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.walk.last {
                    indices.walk.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }

    //TODO other poses

    // TODO Bug Fix multiple key presses
    
}

