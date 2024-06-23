mod camera;
mod player;
mod sprite_animation;
mod movement;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use crate::sprite_animation::SpriteAnimationPlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
