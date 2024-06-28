use bevy::prelude::*;
use bevy::prelude::KeyCode::Space;
use crate::ground_detection::Grounded;
use crate::hit_box::HitBox;

use crate::movement::{MovingObjectBundle, Velocity};
use crate::player_animation::{Animation, PlayerAnimations};
use crate::sprite_animation::{AnimationIndices, AnimationTimer};

const MOVE_SPEED: f32 = 110.;
const FALL_SPEED: f32 = 98.;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct PlayerState();

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_move, handle_jump, handle_fall, player_jump, change_animation).chain())
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
        HitBox(Vec2::splat(32.)),
        Player
    ));
}


fn player_move(
    mut query: Query<(&mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut player_velocity) = query.single_mut();
    if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        player_velocity.x = -MOVE_SPEED;
    } else if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        player_velocity.x = MOVE_SPEED;
    } else {
        player_velocity.x = 0.;
    }
}


#[derive(Debug, Component)]
struct Jump(f32);

fn player_jump(
    mut commands: Commands,
    mut query: Query<(Entity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (player) = query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        commands.entity(player).insert(Jump(100.));
    }
}

fn handle_jump(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transorm, mut jump)) = query.get_single_mut() else { return; };
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transorm.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}

fn handle_fall(
    mut query: Query<(&mut Transform), (With<Player>, Without<Jump>)>,
    time: Res<Time>,
) {
    let Ok((mut transform)) = query.get_single_mut() else { return; };
    if transform.translation.y > 0. {
        transform.translation.y -= time.delta_seconds() * FALL_SPEED;
        if transform.translation.y < 0. {
            transform.translation.y = 0.;
        }
    }
}


fn change_animation(
    mut query: Query<(&mut TextureAtlas, &mut AnimationIndices, &mut Sprite, &mut Handle<Image>), With<Player>>,
    query_jump: Query<(&Grounded, Option<&Jump>), With<Player>>,
    player_animations: Res<PlayerAnimations>,
    input: Res<ButtonInput<KeyCode>>,
    mut last_animation: Local<Animation>,
) {
    let (mut atlas, mut indices, mut sprite, mut texture) = query.single_mut();
    let (on_ground, jump) = query_jump.single();

    change_direction(&input, &mut sprite);

    let current_animation =
        if jump.is_some() {
            Animation::Jump
        } else if !on_ground.0 {
            Animation::Fall
        } else if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyD, KeyCode::ArrowRight]) {
            Animation::Running
        } else {
            Animation::Idle
        };
    if current_animation == *last_animation {
        return;
    }
    if let Some((new_texture, texture_atlas_layout, animation_indices)) = player_animations.get(current_animation.clone()) {
        indices.first = animation_indices.first;
        indices.last = animation_indices.last;
        atlas.index = indices.first;
        atlas.layout = texture_atlas_layout;
        *texture = new_texture;
        *last_animation = current_animation;
    } else {
        error!("Failed to find animation: {:?}", &current_animation);
        return;
    };
}

fn change_direction(input: &Res<ButtonInput<KeyCode>>, sprite: &mut Mut<Sprite>) {
    if input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowRight])
        && !input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::KeyA, KeyCode::ArrowLeft])
        && !input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft])
        && input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        sprite.flip_x = false;
    }
}