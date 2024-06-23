use bevy::prelude::*;

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
            (update_velocity, update_position)
                .chain()
        )
        ;
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in query.iter_mut() {
        transform.translation.x += vel.value.x * time.delta_seconds();
        transform.translation.y += vel.value.y * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}