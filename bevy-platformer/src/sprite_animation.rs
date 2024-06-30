use bevy::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimateOnce;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (animate_sprite, animate_sprite_once));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), Without<AnimateOnce>>,
) {
    for (indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

fn animate_sprite_once(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<AnimateOnce>>,
) {
    for (entity, indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index >= indices.last {
                commands.entity(entity).despawn_recursive()
            } else {
                atlas.index += 1
            }
        }
    }
}
