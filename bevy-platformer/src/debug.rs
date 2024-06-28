use bevy::prelude::*;

use crate::player::{Player, PlayerState};

const DEBUG_FREQUENCY: f32 = 1.;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct DebugTimer{
    timer: Timer
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugTimer{timer: Timer::from_seconds(DEBUG_FREQUENCY, TimerMode::Repeating)})
            .add_systems(Update, debug_player)
        ;
    }
}

fn debug_player(
    query: Query<(&PlayerState), With<Player>>,
    mut debug_timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {
    debug_timer.tick(time.delta());
    if !debug_timer.just_finished() {
        return;
    }
    let Ok(player_state) = query.get_single() else {
        info!("Player not found");
        return;
    };
    info!("Player state is {:?}", player_state);
}