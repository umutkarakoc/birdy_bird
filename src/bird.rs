use std::ops::DerefMut;

use crate::{
    animaton::*,
    game::Score,
    obstacle::{Move, Obstacle, ScoreZone},
    GameState,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::Anchor,
};

pub struct BirdPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_bird)
            .add_systems(OnEnter(GameState::Game), reset_bird)
            .add_systems(
                Update,
                (fall, flap, collision, score).run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (fall, dead_rotate).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(PostUpdate, clean_dust);
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Dead;
#[derive(Component)]
pub struct Dust;

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
            transform: Transform::from_xyz(-200., 0., 2.).with_scale(Vec3::new(0.8, 0.8, 0.8)),
            ..default()
        },
        Animation::new(500, 0, 19),
        Flapping(Timer::from_seconds(0.2, TimerMode::Once)),
        Force(400.0),
    ));
}

fn reset_bird(
    mut commands: Commands,
    bird: Query<Entity, With<Bird>>,
    scorezones: Query<Entity, With<ScoreZone>>,
    obstactles: Query<Entity, With<Obstacle>>,
) {
    commands.insert_resource(Score(0));
    let bird = bird.single();
    commands
        .entity(bird)
        .insert((
            Active,
            Transform::from_xyz(-200., 0., 2.).with_scale(Vec3::new(0.8, 0.8, 0.8)),
            Animation::new(500, 0, 19),
            Flapping(Timer::from_seconds(0.2, TimerMode::Once)),
            Force(400.0),
        ))
        .remove::<Dead>();

    for e in &obstactles {
        commands.entity(e).despawn();
    }
    for e in &scorezones {
        commands.entity(e).despawn();
    }
}

fn fall(
    mut commands: Commands,
    mut bird: Query<
        (Entity, &mut Transform, &mut Force, Option<&mut Flapping>),
        (With<Bird>, With<Active>),
    >,
    time: Res<Time>,
) {
    if bird.is_empty() {
        return;
    }

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

fn flap(
    keys: Res<Input<KeyCode>>,
    mut bird: Query<Entity, (With<Bird>, Without<Dead>)>,
    mut commands: Commands,
) {
    if bird.is_empty() {
        return;
    }
    if keys.just_pressed(KeyCode::Space) {
        let entity = bird.single_mut();
        commands
            .entity(entity)
            .insert(Flapping(Timer::from_seconds(0.2, TimerMode::Once)))
            .insert(Force(400.));
    }
}

fn collision(
    bird: Query<(Entity, &Transform), (With<Bird>, Without<Dead>)>,
    obstactles: Query<&Transform, With<Obstacle>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if bird.is_empty() {
        return;
    }
    let (bird, t) = bird.single();

    for o in &obstactles {
        let collision = collide(
            t.translation,
            Vec2::new(60., 60.),
            o.translation,
            Vec2::new(105., 500.),
        );

        let y = t.translation.y;
        if collision.is_some() || y < -400. || y > 400. {
            commands
                .entity(bird)
                .insert(Dead)
                .insert(Flapping(Timer::from_seconds(0.2, TimerMode::Once)))
                .insert(Force(500.));
            let texture_handle = asset_server.load("dust.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(451.0, 378.0), 5, 4, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            println!("dead");
            // commands.spawn((
            //     Name::new("dead"),
            //     Dust,
            //     Move,
            //     SpriteSheetBundle {
            //         texture_atlas: texture_atlas_handle,
            //         sprite: TextureAtlasSprite::new(0),
            //         transform: t.clone().with_scale(Vec3::ONE * 0.4),
            //         ..default()
            //     },
            //     Animation::new(5000, 0, 20),
            //     NoRepeat,
            // ));
            next_state.set(GameState::MainMenu);
        }
    }
}

fn dead_rotate(mut bird: Query<&mut Transform, (With<Bird>, With<Dead>)>, time: Res<Time>) {
    if bird.is_empty() {
        return;
    }
    let mut bird = bird.single_mut();
    bird.rotate_z(time.delta_seconds() * 10.);
}

fn score(
    bird: Query<(Entity, &Transform), (With<Bird>, Without<Dead>)>,
    scorezones: Query<(Entity, &Transform), With<ScoreZone>>,
    score: Res<Score>,
    mut commands: Commands,
) {
    if bird.is_empty() {
        return;
    }
    let (bird, t) = bird.single();

    for (zone, o) in &scorezones {
        let collision = collide(
            t.translation,
            Vec2::new(40., 40.),
            o.translation,
            Vec2::new(105., 2000.),
        );

        if collision.is_some() {
            commands.insert_resource(Score(score.0 + 1));
            commands.entity(zone).despawn();
        }
    }
}

fn clean_dust(dusts: Query<Entity, (With<Ended>, With<Dust>)>, mut commands: Commands) {
    for dust in &dusts {
        commands.entity(dust).despawn();
    }
}
