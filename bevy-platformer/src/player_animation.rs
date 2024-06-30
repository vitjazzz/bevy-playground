use std::collections::HashMap;
use bevy::math::Vec2;
use bevy::prelude::*;
use crate::sprite_animation::AnimationIndices;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Default)]
pub enum Animation {
    #[default]
    Idle,
    Running,
    Jump,
    Fall,
}

#[derive(Debug, Resource)]
pub struct PlayerAnimations {
    map: HashMap<Animation, (Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices)>,
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
    pub fn add(&mut self, id: Animation, image: Handle<Image>, layout: Handle<TextureAtlasLayout>, indices: AnimationIndices) {
        self.map.insert(id, (image, layout, indices));
    }

    pub fn get(&self, id: Animation) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices)> {
        self.map.get(&id).cloned()
    }
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut animations = PlayerAnimations { map: HashMap::new() };
        let asset_server = world.resource::<AssetServer>();
        let idle_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png").clone();
        let running_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Run (32x32).png").clone();
        let jump_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Jump (32x32).png").clone();
        let fall_texture: Handle<Image> = asset_server.load("Main Characters/Mask Dude/Fall (32x32).png").clone();
        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let idle_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 11, 1, None, None)
        );
        let running_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 12, 1, None, None)
        );
        let jump_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 1, 1, None, None)
        );
        let fall_layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(32.), 1, 1, None, None)
        );

        animations.add(Animation::Idle, idle_texture, idle_layout, AnimationIndices { first: 0, last: 10 });
        animations.add(Animation::Running, running_texture, running_layout, AnimationIndices { first: 0, last: 11 });
        animations.add(Animation::Jump, jump_texture, jump_layout, AnimationIndices{first:0, last:0});
        animations.add(Animation::Fall, fall_texture, fall_layout, AnimationIndices{first:0, last:0});
        animations
    }
}