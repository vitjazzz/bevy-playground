use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Copy,  PartialEq, Eq, Hash, Reflect, Actionlike)]
pub enum PlayerInput {
    Left, Right, Jump,
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            (PlayerInput::Left, KeyCode::KeyA),
            (PlayerInput::Left, KeyCode::ArrowLeft),
            (PlayerInput::Right, KeyCode::KeyD),
            (PlayerInput::Right, KeyCode::ArrowRight),
            (PlayerInput::Jump, KeyCode::Space),
        ]);

        map
    }
}