use bevy::prelude::*;
use crate::movement::{MovingObjectBundle, Velocity};
use crate::sprite_animation::{AnimationIndices, AnimationTimer};

const MOVE_SPEED: f32 = 100.;

#[derive(Debug, Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {first: 0, last: 10};
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        MovingObjectBundle{
            velocity: Velocity::new(Vec2::ZERO),
            ..default()
        },
        Player
        ));
}


fn move_player(
    mut query: Query<(&mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut player_velocity = query.single_mut();
    if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        player_velocity.x = -MOVE_SPEED;
    } else if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        player_velocity.x = MOVE_SPEED;
    } else {
        player_velocity.x = 0.;
    }
}
