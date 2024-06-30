use std::collections::HashMap;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use crate::sprite_animation::AnimationIndices;



extern crate rand;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Default)]
pub enum CollectiblesType {
    #[default]
    Apple,
    Bananas,
    Cherries,
    Kiwi,
    Melon,
    Orange,
    Pineapple,
    Strawberry,
}

impl Distribution<CollectiblesType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CollectiblesType {
        match rng.gen_range(0..=7) {
            0 => CollectiblesType::Apple,
            1 => CollectiblesType::Bananas,
            2 => CollectiblesType::Cherries,
            3 => CollectiblesType::Kiwi,
            4 => CollectiblesType::Melon,
            5 => CollectiblesType::Orange,
            6 => CollectiblesType::Pineapple,
            _ => CollectiblesType::Strawberry,
        }
    }
}

#[derive(Debug, Resource)]
pub struct CollectiblesAnimations {
    map: HashMap<CollectiblesType, (Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices)>,
}

pub struct CollectiblesAnimationPlugin;

impl Plugin for CollectiblesAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CollectiblesAnimations>()
        ;
    }
}

impl CollectiblesAnimations {
    pub fn add(&mut self, id: CollectiblesType, image: Handle<Image>, layout: Handle<TextureAtlasLayout>, indices: AnimationIndices) {
        self.map.insert(id, (image, layout, indices));
    }

    pub fn get(&self, id: CollectiblesType) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices)> {
        self.map.get(&id).cloned()
    }
}

impl FromWorld for CollectiblesAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut animations = CollectiblesAnimations { map: HashMap::new() };
        let asset_server = world.resource::<AssetServer>();
        let apple_texture: Handle<Image> = asset_server.load("Fruits/Apple.png").clone();
        let bananas_texture: Handle<Image> = asset_server.load("Fruits/Bananas.png").clone();
        let cherries_texture: Handle<Image> = asset_server.load("Fruits/Cherries.png").clone();
        let kiwi_texture: Handle<Image> = asset_server.load("Fruits/Kiwi.png").clone();
        let melon_texture: Handle<Image> = asset_server.load("Fruits/Melon.png").clone();
        let orange_texture: Handle<Image> = asset_server.load("Fruits/Orange.png").clone();
        let pineapple_texture: Handle<Image> = asset_server.load("Fruits/Pineapple.png").clone();
        let strawberry_texture: Handle<Image> = asset_server.load("Fruits/Strawberry.png").clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let collectibles_layout = layouts.add(TextureAtlasLayout::from_grid(Vec2::splat(32.), 17, 1, None, None));

        animations.add(CollectiblesType::Apple, apple_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Bananas, bananas_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Cherries, cherries_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Kiwi, kiwi_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Melon, melon_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Orange, orange_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Pineapple, pineapple_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations.add(CollectiblesType::Strawberry, strawberry_texture, collectibles_layout.clone(), AnimationIndices { first: 0, last: 16 });
        animations
    }
}