use bevy::prelude::*;
use crate::hit_box::HitBox;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map)
        ;
    }
}

fn spawn_map(
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * 16.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 5.)),
                color: Color::WHITE,
                ..default()
            },
            ..default()
        },
        HitBox(Vec2::new(200., 5.)),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100., 25., 0.)),
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        HitBox(Vec2::new(32., 32.)),
    ));
}