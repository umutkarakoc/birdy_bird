use std::time::Duration;

use crate::{animaton::*, GameState};
use bevy::{prelude::*, reflect::TypeData, sprite::Anchor};

pub struct ObstaclePlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0., TimerMode::Repeating)))
            .add_systems(Update, (spawner, slide).in_set(GameState::Game));
    }
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct SpawnTimer(Timer);

fn spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    spawner.0.tick(time.delta());

    if spawner.0.finished() {
        spawner.0.set_duration(Duration::from_millis(1500));
        let texture = asset_server.load("obstacle.png");

        let r = rand::random::<f32>();
        let r = r * 300.;

        commands.spawn((
            Name::new("obstactle"),
            Obstacle,
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(700., 520. - r, 1.),
                ..default()
            },
        ));
        commands.spawn((
            Name::new("obstactle"),
            Obstacle,
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                transform: Transform::from_xyz(700., -220. - r, 1.),
                ..default()
            },
        ));
    }
}

fn slide(
    mut commands: Commands,
    mut obstacles: Query<(Entity, &mut Transform), With<Obstacle>>,
    time: Res<Time>,
) {
    for (entity, mut t) in obstacles.iter_mut() {
        t.translation.x -= 300. * time.delta_seconds();
    }
}
