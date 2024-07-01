use bevy::prelude::*;
use bevy_editor_pls::egui::Shape::Vec;
use crate::ground_detection::Grounded;
use crate::hit_box;
use crate::hit_box::HitBox;
use crate::map::Obstacle;
use crate::player::Player;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(value: Vec2) -> Self{
        Self {value}
    }
}


#[derive(Component, Debug, Default)]
pub struct Acceleration {
    pub value: Vec2
}

impl Acceleration {
    pub fn new(value: Vec2) -> Self{
        Self {value}
    }
}

#[derive(Bundle, Default)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_player_position)
                .chain()
        )
        ;
    }
}


fn update_player_position(
    mut query: Query<(&Velocity, &mut Transform, &Grounded, &HitBox), With<Player>>,
    hitboxes: Query<(&HitBox, &Transform), (With<Obstacle>, Without<Player>)>,
    time: Res<Time>,
) {
    let ( velocity, mut p_offset, grounded, p_hitbox) = query.single_mut();

    if velocity.value.x == 0. && velocity.value.y == 0. {
        return;
    }

    let delta_x = velocity.value.x * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32);

    let new_position = p_offset.translation + Vec3::X * delta_x;
    for (hitbox, offset) in &hitboxes {
        if hit_box::check_hit(*p_hitbox, new_position, *hitbox, offset.translation) {
            return;
        }
    }
    p_offset.translation = new_position;
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}