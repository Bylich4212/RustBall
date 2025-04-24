use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

pub fn update_turn_text(
    turn_state: Res<TurnState>,
    mut query: Query<&mut Text, With<TurnText>>,
) {
    if turn_state.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Turno: Jugador {}", turn_state.current_turn);
    }
}

pub fn update_score_text(
    scores: Res<Scores>,
    mut texts: Query<&mut Text, With<ScoreText>>,
) {
    if scores.is_changed() {
        let mut text = texts.single_mut();
        text.sections[0].value = format!("P1: {}  -  P2: {}", scores.left, scores.right);
    }
}

pub fn update_power_bar(
    turn_state: Res<TurnState>,
    mut query: Query<&mut Style, With<PowerBar>>,
) {
    if let Ok(mut style) = query.get_single_mut() {
        style.width = Val::Px(200.0 * turn_state.power);
    }
}
