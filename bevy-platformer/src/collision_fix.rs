use bevy::prelude::*;
use crate::hit_box;
use crate::hit_box::HitBox;
use crate::player::Player;

const MAX_FIX: i32 = 100;

pub struct CollisionFixPlugin;

impl Plugin for CollisionFixPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixCollisionTimer(Timer::from_seconds(1., TimerMode::Repeating)))
            .add_systems(Update, fix_collision)
        ;
    }
}

#[derive(Debug, Resource)]
pub struct FixCollisionTimer(Timer);

fn fix_collision(
    mut query: Query<(&mut Transform, &HitBox), With<Player>>,
    hitboxes: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
    mut fix_collision_timer: ResMut<FixCollisionTimer>,
) {
    fix_collision_timer.0.tick(time.delta());
    if !fix_collision_timer.0.just_finished() {
        return;
    }

    let Ok((mut p_offset, p_hitbox)) = query.get_single_mut() else { return; };

    if no_collision(p_offset.translation, p_hitbox, &hitboxes) {
        return;
    }

    let mut result_position = p_offset.translation;
    for i in 1..MAX_FIX {
        let i = i as f32;
        let new_position = p_offset.translation + Vec3::Y * i;
        if no_collision(new_position, p_hitbox, &hitboxes) {
            result_position = new_position;
        }
        let new_position = p_offset.translation - Vec3::Y * i;
        if no_collision(new_position, p_hitbox, &hitboxes) {
            result_position = new_position;
        }
        let new_position = p_offset.translation + Vec3::X * i;
        if no_collision(new_position, p_hitbox, &hitboxes) {
            result_position = new_position;
        }
        let new_position = p_offset.translation - Vec3::X * i;
        if no_collision(new_position, p_hitbox, &hitboxes) {
            result_position = new_position;
        }
    }
    info!("Fixed collision from {:?} to {:?}", p_offset.translation, result_position);
    p_offset.translation = result_position;
}

fn no_collision(p_position: Vec3, p_hitbox : &HitBox, hitboxes: &Query<(&HitBox, &Transform), Without<Player>>) -> bool {
    for (hitbox, offset) in hitboxes {
        if hit_box::check_hit(*p_hitbox, p_position, *hitbox, offset.translation) {
            return false
        }
    }
    return true
}