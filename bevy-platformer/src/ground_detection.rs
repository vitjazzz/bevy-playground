use bevy::prelude::*;
use crate::player::Player;


pub struct GroundDetectionPlugin;

impl Plugin for GroundDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ground_detection)
        ;
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Grounded(pub bool);

fn ground_detection(
    mut query: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let ( position, mut on_ground) = query.single_mut();
    let current = if position.translation.y == last.translation.y {
        true
    } else {
        false
    };
    if current != on_ground.0 {
        on_ground.0 = current;
    }
    *last = *position;
}