use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(ms: u64) -> AnimationTimer {
        AnimationTimer(Timer::new(Duration::from_millis(ms), TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct AnimationIndex {
    pub start: usize,
    pub end: usize,
}

#[derive(Component)]
pub struct NoRepeat;

#[derive(Component)]
pub struct Ended;

impl AnimationIndex {
    pub fn new(start: usize, end: usize) -> AnimationIndex {
        AnimationIndex { start, end }
    }
}

#[derive(Bundle)]
pub struct Animation {
    pub timer: AnimationTimer,
    pub index: AnimationIndex,
}
impl Animation {
    pub fn new(duration: u64, start: usize, end: usize) -> Animation {
        Animation {
            timer: AnimationTimer::new(duration / (end - start) as u64),
            index: AnimationIndex::new(start, end),
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &AnimationIndex,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        Option<&NoRepeat>,
    )>,
    mut commands: Commands,
) {
    for (entity, indices, mut timer, mut sprite, norepeat) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut next = sprite.index + 1;

            if next == indices.end && norepeat.is_some() {
                commands.entity(entity).insert(Ended);
                return;
            }
            if next > indices.end {
                next = 0;
            }
            sprite.index = next;
        }
    }
}
