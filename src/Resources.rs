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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Formation {
    Rombo1211,
    Muro221,
    Ofensiva113,
    Diamante2111,
}

#[derive(Resource, Debug)]
pub struct PlayerFormations {
    pub player1: Option<Formation>,
    pub player2: Option<Formation>,
}

/// Estados globales del juego
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    FormationSelection,
    InGame,
    FormationChange,
    GoalScored, // ✅ nuevo estado temporal
}

#[derive(Component)]
pub struct PowerBarBackground;

#[derive(Resource)]
pub struct TeamSelectionMusic(pub Handle<AudioSource>);











