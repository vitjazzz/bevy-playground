use std::collections::HashMap;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    Idle,
    Running,
}

#[derive(Debug, Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<Animation, (Handle<Image>, Handle<TextureAtlasLayout>)>,
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAnimations>()
        ;
    }
}

impl PlayerAnimations {
    pub fn add(&mut self, id: Animation, image: Handle<Image>, layout: Handle<TextureAtlasLayout>) {
        self.map.insert(id, (image, layout));
    }

    pub fn get(&self, id: Animation) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>)> {
        self.map.get(&id).cloned()
    }
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut animations = PlayerAnimations {map: HashMap::new()};
        let asset_server = world.resource::<AssetServer>();
        let idle_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
        let running_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Run (32x32).png");
        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let idle_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 11, 1, None, None)
        );
        let running_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 12, 1, None, None)
        );

        animations.add(Animation::Idle, idle_texture, idle_layout);
        animations.add(Animation::Running, running_texture, running_layout);
        animations
    }
}