use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(Update, game_state_input_events)
            .add_systems(Update, transition_to_in_game.run_if(in_state(GameState::GameOver)))
        ;
    }
}

pub fn game_state_input_events(mut next_state: ResMut<NextState<GameState>>,
                               state: Res<State<GameState>>,
                               keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => ()
        }
    }
}

pub fn transition_to_in_game(mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::InGame)
}