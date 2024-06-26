use bevy::prelude::*;
use crate::hit_box::HitBox;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TerrainSprites>()
            .add_systems(Startup, spawn_map)
        ;
    }
}


#[derive(Component)]
pub struct Obstacle;

fn spawn_map(
    mut commands: Commands,
    terrain: Res<TerrainSprites>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * 16.),
            texture: terrain.get_image(),
            sprite: Sprite{
                color: Color::WHITE,
                custom_size: Some(Vec2::new(168., 16.)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: terrain.get_layout(),
            index: TerrainType::GoldStraight as usize,
            ..default()
        },
        HitBox(Vec2::new(200., 16.)),
        Obstacle,
    ))
        .with_children(|p| {
            p.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::X * 92.),
                    texture: terrain.get_image(),
                    sprite: Sprite{
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(16., 16.)),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: terrain.get_layout(),
                    index: TerrainType::GoldRightEnd as usize,
                    ..default()
                },
                ));
            p.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::NEG_X * 92.),
                    texture: terrain.get_image(),
                    sprite: Sprite{
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(16., 16.)),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: terrain.get_layout(),
                    index: TerrainType::GoldLeftEnd as usize,
                    ..default()
                },
                ));
        });
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100., 25., 0.)),
            texture: terrain.get_image(),
            sprite: Sprite{
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            ..Default::default()
        },
        TextureAtlas {
            layout: terrain.get_layout(),
            index: TerrainType::GoldFullEnd as usize,
            ..default()
        },
        HitBox(Vec2::new(32., 32.)),
        Obstacle,
    ));
}

#[derive(Debug, Resource)]
struct TerrainSprites {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl TerrainSprites {
    fn get_image(&self) -> Handle<Image> {
        self.image.clone()
    }
    fn get_layout(&self) -> Handle<TextureAtlasLayout> {
        self.layout.clone()
    }
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self { image, layout }
    }
}

impl FromWorld for TerrainSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let terrain_texture: Handle<Image> = asset_server.load("Terrain/Terrain (16x16).png").clone();
        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = texture_atlas_layouts.add(
            TextureAtlasLayout::from_grid(Vec2::splat(16.), 22, 11, None, None)
        );

        TerrainSprites::new(terrain_texture, layout)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainType {
    GoldLeftEnd = 193,
    GoldStraight = 194,
    GoldRightEnd = 195,

    GoldFullEnd = 215,
}