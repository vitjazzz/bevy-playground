use bevy::ecs::schedule::run_enter_schedule;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::{Collider, CollisionDamage};
use crate::health::Health;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use crate::state::GameState;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.0;

const MISSILE_SPEED: f32 = 35.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 0.5;
const MISSILE_HEALTH: f32 = 0.1;
const MISSILE_COLLISION_DAMAGE: f32 = 3.5;


#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_spaceship)
            .add_systems(OnEnter(GameState::GameOver), spawn_spaceship)
            .add_systems(
                Update,
                (
                    spaceship_movement_controls,
                    spaceship_weapons_controls,
                    spaceship_shield_controls,
                )
                    .chain()
                    .in_set(InGameSet::UserInput),
            )
            .add_systems(Update, spaceship_destroyed.in_set(InGameSet::EntityUpdates))
        ;
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
    ));
}

fn spaceship_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
                               keyboard_input: Res<Input<KeyCode>>,
                               time: Res<Time>) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        rotation -= SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation += SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement -= SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement += SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll -= SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll += SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapons_controls(mut commands: Commands,
                              scene_assets: Res<SceneAssets>,
                              mut query: Query<(&mut Transform), With<Spaceship>>,
                              keyboard_input: Res<Input<KeyCode>>) {
    let Ok(transform) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR
                    ),
                    ..default()
                },
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
            },
            SpaceshipMissile,
            Health::new(MISSILE_HEALTH),
            CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
        ));
    }
}

fn spaceship_shield_controls(mut commands: Commands,
                             mut query: Query<(Entity), With<Spaceship>>,
                             keyboard_input: Res<Input<KeyCode>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(entity).insert(SpaceshipShield);
    }
}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>
) {
    if query.is_empty() {
        next_state.set(GameState::GameOver);
    }
}