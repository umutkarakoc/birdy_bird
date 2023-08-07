use std::ops::DerefMut;

use crate::{animaton::*, GameState};
use bevy::{prelude::*, sprite::Anchor};

pub struct BirdPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), create_bird)
            .add_systems(Update, (fall, flap).run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Flapping(Timer);

#[derive(Component)]
pub struct Force(f32);

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
            transform: Transform::from_xyz(-200., 0., 0.).with_scale(Vec3::new(0.8, 0.8, 0.8)),
            ..default()
        },
        Animation::new(500, 0, 19),
        Force(0.0),
    ));
}

fn fall(
    mut commands: Commands,
    mut bird: Query<(Entity, &mut Transform, &mut Force, Option<&mut Flapping>), With<Bird>>,
    time: Res<Time>,
) {
    let (entity, mut t, mut f, flapping) = bird.single_mut();
    if let Some(mut flapping) = flapping {
        let timer = flapping.deref_mut();
        f.0 -= 1000. * time.delta_seconds();
        t.translation.y += f.0 * time.delta_seconds();
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands
                .entity(entity)
                .remove::<Flapping>()
                .insert(Force(-1.));
        }
    } else {
        t.translation.y += f.0 * time.delta_seconds();
        f.0 -= 1000. * time.delta_seconds();
    }
}

fn flap(keys: Res<Input<KeyCode>>, mut bird: Query<Entity, With<Bird>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) {
        let entity = bird.single_mut();
        commands
            .entity(entity)
            .insert(Flapping(Timer::from_seconds(0.2, TimerMode::Once)))
            .insert(Force(400.));
    }
}
