use bevy::prelude::*;

#[derive(Default)]
pub struct GamepadState {
    pub cam: Vec2,
    pub joy: Vec2,
    pub jump: bool,
}
