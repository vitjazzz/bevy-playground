mod movement;
mod spaceship;
mod debug;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;
mod schedule;
mod state;
mod health;

use bevy::prelude::*;
use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::debug::DebugPlugin;
use crate::despawn::DespawnPlugin;
use crate::movement::{MovementPlugin};
use crate::schedule::SchedulePlugin;
use crate::spaceship::*;
use crate::state::StatePlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight{
            color: Color::default(),
            brightness: 0.75
        })
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
