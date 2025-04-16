use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TurnState {
    pub current_turn: usize,
    pub in_motion: bool,
    pub selected_entity: Option<Entity>,
    pub aim_direction: Vec2,
    pub power: f32,
}

#[derive(Resource, Default)]
pub struct Scores {
    pub left: u32,
    pub right: u32,
}



