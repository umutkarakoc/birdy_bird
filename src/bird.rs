use bevy::{prelude::*, sprite::Anchor};

use crate::{animaton::*, GameState};

pub struct BirdPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), create_bird)
            .add_systems(Update, (fly).in_set(GameState::Game));
    }
}

#[derive(Component)]
pub struct Bird;

fn create_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("spawn bird");
    let texture_handle = asset_server.load("bird.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(216.0, 150.0), 4, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Name::new("bird"),
        Bird,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(-200., 0., 0.),
            ..default()
        },
        Animation::new(500, 0, 19),
    ));
}

fn fly() {}
