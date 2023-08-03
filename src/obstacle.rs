use crate::{animaton::*, GameState};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

pub struct BirdPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), create_bird)
            .add_systems(Update, (fly).in_set(GameState::Game));
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct FlyTimer(Timer);

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
        RigidBody::KinematicVelocityBased,
        Collider::capsule_x(50.0, 25.),
        Velocity {
            linvel: Vec2::new(0.0, -100.0),
            angvel: 0.0,
        },
    ));
}

fn fly(
    mut commands: Commands,
    mut bird: Query<(Entity, &mut Velocity, Option<&mut FlyTimer>), With<Bird>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (entity, mut vel, fly) in bird.iter_mut() {
        if let Some(mut fly) = fly {
            fly.0.tick(time.delta());
            if fly.0.finished() {
                commands.entity(entity).remove::<FlyTimer>();
            }
            vel.linvel = Vec2::new(0.0, vel.linvel.y + (-700. * time.delta_seconds()));
        } else {
            if keys.just_pressed(KeyCode::Space) {
                vel.linvel = Vec2::new(0.0, 400.0);
                commands
                    .entity(entity)
                    .insert(FlyTimer(Timer::from_seconds(0.05, TimerMode::Once)));
            } else {
            }
            vel.linvel = Vec2::new(0.0, vel.linvel.y + (-700. * time.delta_seconds()));
        }
    }
}
