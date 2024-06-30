use bevy::prelude::*;
use crate::collectibles_animation::{CollectiblesAnimations, CollectiblesType};
use crate::hit_box::{check_hit, HitBox};
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
            HitBox(Vec2::splat(8.)),
            Collectible,
            ));
    }
}

fn collect(
    mut commands: Commands,
    player: Query<(&Transform, &HitBox), With<Player>>,
    collectibles: Query<(Entity, &Transform, &HitBox), With<Collectible>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let Ok((p_offset, p_hitbox)) = player.get_single() else { return; };
    for (collectible, c_offset, c_hitbox) in &collectibles {
        if check_hit(*p_hitbox, p_offset.translation, *c_hitbox, c_offset.translation) {
            commands.entity(collectible).despawn_recursive();

            let texture: Handle<Image> = asset_server.load("Fruits/Collected.png").clone();
            let layout = layouts.add(TextureAtlasLayout::from_grid(Vec2::splat(32.), 5, 1, None, None));
            commands.spawn((
                SpriteBundle{
                    transform: Transform::from_translation(c_offset.translation),
                    texture,
                    ..default()
                },
                TextureAtlas{
                    layout,
                    index: 0,
                    ..default()
                },
                AnimationIndices::new(0, 4),
                AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
                AnimateOnce,
                ));
        }
    }
}
