use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::resources::AppState; // Asegúrate de importar AppState
use bevy::ecs::schedule::NextState; // Necesario para cambiar de estado

pub fn detect_goal(
    mut collision_events: EventReader<CollisionEvent>,
    goals: Query<(&GoalZone, Entity)>,
    balls: Query<Entity, With<Ball>>,
    mut goal_events: EventWriter<GoalEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _) = event {
            for (goal, goal_entity) in &goals {
                for ball_entity in &balls {
                    if (*a == goal_entity && *b == ball_entity)
                        || (*b == goal_entity && *a == ball_entity)
                    {
                        goal_events.send(GoalEvent {
                            scored_by_left: !goal.is_left,
                        });
                    }
                }
            }
        }
    }
}

pub fn handle_goal(
    mut goal_events: EventReader<GoalEvent>,
    mut scores: ResMut<Scores>,
    mut turn_state: ResMut<TurnState>,
    mut sprites: Query<&mut Sprite>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>, // ✅ importante
) {
    for event in goal_events.read() {
        if event.scored_by_left {
            scores.left += 1;
            println!("Gol para el jugador izquierdo! Puntos: {}", scores.left);
        } else {
            scores.right += 1;
            println!("Gol para el jugador derecho! Puntos: {}", scores.right);
        }

        // ✅ Reiniciar lógica de turno
        if let Some(entity) = turn_state.selected_entity {
            if let Ok(mut sprite) = sprites.get_mut(entity) {
                sprite.color = Color::WHITE;
            }
            commands.entity(entity).remove::<TurnControlled>();
        }

        turn_state.in_motion = false;
        turn_state.selected_entity = None;
        turn_state.aim_direction = Vec2::ZERO;
        turn_state.power = 0.0;
        turn_state.current_turn = turn_state.current_turn % 2 + 1;

        // ✅ Cambiar al estado para re-seleccionar formaciones
        next_state.set(AppState::FormationChange);
    }
}
