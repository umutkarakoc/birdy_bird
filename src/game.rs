use std::rc::Rc;

use bevy::{prelude::*, sprite::Anchor};

use crate::GameState;

pub struct GamePlugin;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(i32);

#[derive(Component)]
pub struct Level;

#[derive(Component)]
pub struct Background;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, create_env)
            .add_systems(Update, (start_game).in_set(GameState::MainMenu))
            .add_systems(Update, (move_bg, log_score).in_set(GameState::Game));
    }
}

fn start_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("start game");
        next_state.set(GameState::Game);
    }
}

fn create_env(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("level"),
            Level,
            GlobalTransform::default(),
            ComputedVisibility::default(),
            Visibility::Visible,
            Transform::from_xyz(0.0, 0.0, -1.0),
        ))
        .with_children(|level| {
            level.spawn((
                Name::new("bg1"),
                Background,
                SpriteBundle {
                    texture: asset_server.load("bg.png"),
                    transform: Transform::from_xyz(-400.0, 0.0, 0.0),
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            ));
            // level.spawn((
            //     Name::new("bg2"),
            //     Background,
            //     SpriteBundle {
            //         texture: asset_server.load("bg.png"),
            //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //         sprite: Sprite {
            //             anchor: Anchor::Center,
            //             ..default()
            //         },
            //         ..default()
            //     },
            // ));
            // level.spawn((
            //     Name::new("bg3"),
            //     Background,
            //     SpriteBundle {
            //         texture: asset_server.load("bg.png"),
            //         transform: Transform::from_xyz(400.0, 0.0, 0.0),
            //         sprite: Sprite {
            //             anchor: Anchor::Center,
            //             ..default()
            //         },
            //         ..default()
            //     },
            // ));
            // level.spawn((
            //     Name::new("bg4"),
            //     Background,
            //     SpriteBundle {
            //         texture: asset_server.load("bg.png"),
            //         transform: Transform::from_xyz(800.0, 0.0, 0.0),
            //         sprite: Sprite {
            //             anchor: Anchor::Center,
            //             ..default()
            //         },
            //         ..default()
            //     },
            // ));
            // level.spawn((
            //     Name::new("bg5"),
            //     Background,
            //     SpriteBundle {
            //         texture: asset_server.load("bg.png"),
            //         transform: Transform::from_xyz(1200.0, 0.0, 0.0),
            //         sprite: Sprite {
            //             anchor: Anchor::Center,
            //             ..default()
            //         },
            //         ..default()
            //     },
            // ));
        });
}

fn move_bg(mut backgrounds: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    let dt = time.delta_seconds();
    let d = 300.0 * dt;
    for mut b in backgrounds.iter_mut() {
        b.translation.x -= d;
        if b.translation.x < -800.0 {
            b.translation.x += 1200.0
        }
    }
}

fn log_score(score: Res<Score>) {
    if score.is_changed() {
        println!("score: {}", score.0);
    }
}
