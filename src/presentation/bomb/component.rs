use bevy::prelude::*;

use crate::{presentation::{position::Position, window::Window}, view_model_event::bomb_view_model_event::BombViewModelEvent};

// BombのComponent
#[derive(Component)]
pub struct Bomb;

impl Bomb {
    pub fn calc_position(self: &Self, _event: &BombViewModelEvent, _window: &Window) -> Position {
        Position::new(-100.0, 0.0)
    }
}
