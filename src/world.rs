use std::rc::Rc;

use bevy::{prelude::*, sprite::Anchor};

use crate::GameState;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Move(f32);

#[derive(Component)]
pub struct Reset(f32, f32);

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_env)
            .add_systems(Update, move_items);
    }
}

fn create_env(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("sky"),
            Move(5.),
            GlobalTransform::default(),
            ComputedVisibility::default(),
            Visibility::Visible,
            Transform::from_xyz(0.0, 43.0, -1.0),
        ))
        .with_children(|level| {
            level.spawn((
                Name::new("sky1"),
                Move(50.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("sky.png"),
                    transform: Transform::from_xyz(-400.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("sky"),
                Move(50.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("sky.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("sky"),
                Move(50.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("sky.png"),
                    transform: Transform::from_xyz(400.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("sky"),
                Move(50.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("sky.png"),
                    transform: Transform::from_xyz(800.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("sky"),
                Move(50.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("sky.png"),
                    transform: Transform::from_xyz(1200.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            Name::new("ground"),
            GlobalTransform::default(),
            ComputedVisibility::default(),
            Visibility::Visible,
            Transform::from_xyz(0.0, -272.0, -1.0),
        ))
        .with_children(|level| {
            level.spawn((
                Name::new("ground1"),
                Move(300.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(-400.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("ground1"),
                Move(300.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("groun2"),
                Move(300.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(400.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("groun3"),
                Move(300.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(800.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    ..default()
                },
            ));
            level.spawn((
                Name::new("ground4"),
                Move(300.),
                Reset(-800., 1200.),
                SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(1200.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn move_items(mut items: Query<(&mut Transform, &Move, &Reset)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut t, Move(speed), Reset(reset_at, reset_to)) in items.iter_mut() {
        let d = speed * dt;
        t.translation.x -= d;
        if t.translation.x < *reset_at {
            t.translation.x += *reset_to - *reset_at;
        }
    }
}