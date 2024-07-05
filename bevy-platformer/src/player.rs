use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::ground_detection::Grounded;
use crate::hit_box;
use crate::hit_box::HitBox;
use crate::user_input::PlayerInput;
use crate::map::Obstacle;

use crate::movement::{MovingObjectBundle, Velocity};
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
            .add_systems(Update, (player_move, handle_jump, handle_fall).chain())
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
        MovingObjectBundle {
            velocity: Velocity::new(Vec2::ZERO),
            ..default()
        },
        Grounded(true),
        HitBox(Vec2::new(18., 30.)),
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..default()
        },
        Player
    ));
}


fn player_move(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &Grounded, &ActionState<PlayerInput>), With<Player>>,
    query_double_jump: Query<Option<&DoubleJumpAvailable>, With<Player>>,
) {
    let (player, mut player_velocity, grounded, input) = query.single_mut();
    let double_jump_available = query_double_jump.single();

    if input.just_pressed(&PlayerInput::Jump) {
        if grounded.0 {
            commands.entity(player).insert(Jump { speed: 100., is_double_jump: false });
            commands.entity(player).insert(DoubleJumpAvailable);
        } else if double_jump_available.is_some() {
            commands.entity(player).insert(Jump { speed: 100., is_double_jump: true });
            commands.entity(player).remove::<DoubleJumpAvailable>();
        }
    } else if input.pressed(&PlayerInput::Left) {
        player_velocity.x = -MOVE_SPEED;
    } else if input.pressed(&PlayerInput::Right) {
        player_velocity.x = MOVE_SPEED;
    } else {
        player_velocity.x = 0.;
    }
}


#[derive(Debug, Component)]
pub struct Jump {
    speed: f32,
    pub is_double_jump: bool,
}

#[derive(Debug, Component)]
pub struct DoubleJumpAvailable;

fn handle_jump(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Jump, &HitBox, &ActionState<PlayerInput>), With<Player>>,
    hitboxes: Query<(&HitBox, &Transform), (With<Obstacle>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok((player, mut p_offset, mut jump, p_hitbox, input)) = query.get_single_mut() else { return; };
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.speed);

    let new_position = p_offset.translation + Vec3::Y * jump_power;
    for (hitbox, offset) in &hitboxes {
        if hit_box::check_hit(*p_hitbox, new_position, *hitbox, offset.translation) {
            commands.entity(player).remove::<Jump>();
            return;
        }
    }
    p_offset.translation = new_position;

    jump.speed -= if input.pressed(&PlayerInput::Jump) { jump_power } else { jump_power * 2. };
    if jump.speed <= 0. {
        commands.entity(player).remove::<Jump>();
    }
}

fn handle_fall(
    mut query: Query<(&mut Transform, &HitBox), (With<Player>, Without<Jump>)>,
    hitboxes: Query<(&HitBox, &Transform), (With<Obstacle>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok((mut p_offset, p_hitbox)) = query.get_single_mut() else { return; };
    let new_position = p_offset.translation - Vec3::Y * time.delta_seconds() * FALL_SPEED;
    for (hitbox, offset) in &hitboxes {
        if hit_box::check_hit(*p_hitbox, new_position, *hitbox, offset.translation) {
            return;
        }
    }
    p_offset.translation = new_position;
}
