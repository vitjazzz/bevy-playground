use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::collectibles_animation::{CollectiblesAnimations, CollectiblesType};
use crate::player::Player;
use crate::sprite_animation::{AnimateOnce, AnimationIndices, AnimationTimer};

const MAX_COLLECTIBLES: usize = 1;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CollectiblesState::new())
            .insert_resource(PossibleLocations::new())
            .add_systems(Update, (spawn_collectible, collect).chain())
        ;
    }
}


#[derive(Resource)]
struct CollectiblesState {
    last_index: usize,
}

#[derive(Resource)]
struct PossibleLocations(Vec<Vec3>);

impl PossibleLocations {
    pub fn new() -> Self {
        Self(vec![
            Vec3::new(100., 75., 0.),
            Vec3::new(-100., 55., 0.),
            Vec3::new(40., 95., 0.),
        ])
    }
}

impl CollectiblesState {
    pub fn new() -> Self {
        Self { last_index: 0 }
    }
}

#[derive(Component)]
struct Collectible;

fn spawn_collectible(
    mut commands: Commands,
    mut collectibles_state: ResMut<CollectiblesState>,
    collectibles: Query<(Entity,), With<Collectible>>,
    collectibles_animations: Res<CollectiblesAnimations>,
    possible_locations: Res<PossibleLocations>,
) {
    let existing_collectibles = collectibles.iter().len();
    if existing_collectibles >= MAX_COLLECTIBLES {
        return;
    }
    for i in existing_collectibles..MAX_COLLECTIBLES {
        let next_location_index = (collectibles_state.last_index + 1) % possible_locations.0.len();
        let next_location = possible_locations.0.get(next_location_index).unwrap().clone();
        let collectibles_type: CollectiblesType = rand::random();
        let Some((texture, layout, indices)) = collectibles_animations.get(collectibles_type) else {
            error!("Failed to find collectibles animation for {:?}", collectibles_type);
            continue
        };
        collectibles_state.last_index = next_location_index;
        commands.spawn((
            SpriteBundle{
                transform: Transform::from_translation(next_location),
                texture,
                ..default()
            },
            TextureAtlas{
                layout,
                index: indices.first,
                ..default()
            },
            indices,
            AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
            Collider::ball(8.),
            RigidBody::Fixed,
            Sensor,
            Collectible,
            ));
    }
}

fn collect(
    mut commands: Commands,
    player: Query<(Entity), With<Player>>,
    collectibles: Query<(&Transform), With<Collectible>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(player) = player.get_single() else { return; };
    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(player) {
        if !intersecting { continue }
        if let Ok(c_position) = collectibles.get(collider1) {
            commands.entity(collider1).despawn_recursive();
            animate_collection(&mut commands, &mut layouts, &asset_server, c_position);
        }
        if let Ok(c_position) = collectibles.get(collider2) {
            commands.entity(collider2).despawn_recursive();
            animate_collection(&mut commands, &mut layouts, &asset_server, c_position);
        }
    }
}

fn animate_collection(commands: &mut Commands, layouts: &mut ResMut<Assets<TextureAtlasLayout>>, asset_server: &Res<AssetServer>, c_position: &Transform) {

    let texture: Handle<Image> = asset_server.load("Fruits/Collected.png").clone();
    let layout = layouts.add(TextureAtlasLayout::from_grid(Vec2::splat(32.), 5, 1, None, None));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(c_position.translation),
            texture,
            ..default()
        },
        TextureAtlas {
            layout,
            index: 0,
            ..default()
        },
        AnimationIndices::new(0, 4),
        AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
        AnimateOnce,
    ));
}
