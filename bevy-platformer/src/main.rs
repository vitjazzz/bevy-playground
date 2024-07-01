use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::camera::CameraPlugin;
use crate::collectibles::CollectiblesPlugin;
use crate::collectibles_animation::CollectiblesAnimationPlugin;
use crate::collision_fix::CollisionFixPlugin;
use crate::ground_detection::GroundDetectionPlugin;
use crate::user_input::PlayerInput;
use crate::map::MapPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use crate::player_animation::PlayerAnimationPlugin;
use crate::sprite_animation::SpriteAnimationPlugin;

mod camera;
mod player;
mod sprite_animation;
mod movement;
mod player_animation;
mod debug;
mod hit_box;
mod ground_detection;
mod map;
mod collision_fix;
mod collectibles;
mod collectibles_animation;
mod user_input;

fn main() {
    App::new()
        // Bevy built-ins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // third-party plugins
        .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(InputManagerPlugin::<PlayerInput>::default())
        // my plugins
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerAnimationPlugin)
        .add_plugins(GroundDetectionPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(CollisionFixPlugin)
        .add_plugins(CollectiblesAnimationPlugin)
        .add_plugins(CollectiblesPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
