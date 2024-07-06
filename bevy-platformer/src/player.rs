use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::ground_detection::Grounded;
use crate::user_input::PlayerInput;

use crate::player_animation::{Animation, PlayerAnimations};
use crate::sprite_animation::{AnimationTimer};

const MOVE_SPEED: f32 = 110.;
const FALL_SPEED: f32 = 140.;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct PlayerState();

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_move).chain())
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    player_animations: Res<PlayerAnimations>,
) {
    let Some((texture, texture_atlas_layout, animation_indices)) = player_animations.get(Animation::Idle)
        else {
            error!("Failed to find animation: Idle");
            return;
        };
    commands.spawn((
        SpriteBundle {
            // transform: Transform::from_scale(Vec3::splat(3.)),
            transform: Transform::from_translation(Vec3::Y * 16.),
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
        Grounded(true),
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..default()
        },
        Jump{currently_jumping: false, is_double_jump: false},
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::cuboid(9., 15.),
        LockedAxes::ROTATION_LOCKED_Z,
        Player,
    ));
}


fn player_move(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &ActionState<PlayerInput>, &Grounded, &mut Jump), With<Player>>,
    query_double_jump: Query<Option<&DoubleJumpAvailable>, With<Player>>,
) {
    let (player, mut player_velocity, input, grounded, mut jump) = query.single_mut();
    let double_jump_available = query_double_jump.single();

    if input.just_pressed(&PlayerInput::Jump) {
        if grounded.0 && player_velocity.linvel.y == 0. {
            player_velocity.linvel.y = 100.;
            jump.currently_jumping = true;
            jump.is_double_jump = false;
            commands.entity(player).insert(DoubleJumpAvailable);
        } else if double_jump_available.is_some() {
            player_velocity.linvel.y = 100.;
            jump.currently_jumping = true;
            jump.is_double_jump = true;
            commands.entity(player).remove::<DoubleJumpAvailable>();
        }
    } else if input.pressed(&PlayerInput::Left) {
        player_velocity.linvel.x = -MOVE_SPEED;
    } else if input.pressed(&PlayerInput::Right) {
        player_velocity.linvel.x = MOVE_SPEED;
    } else {
        player_velocity.linvel.x = 0.;
    }
}


#[derive(Debug, Component, Reflect)]
pub struct Jump {
    pub currently_jumping: bool,
    pub is_double_jump: bool,
}

#[derive(Debug, Component)]
pub struct DoubleJumpAvailable;
