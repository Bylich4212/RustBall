use bevy::prelude::*;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy_rapier2d::prelude::*;
use bevy::gizmos::gizmos::Gizmos; //nuevo

use crate::components::*;
use crate::resources::*;
use crate::events::*;

pub fn auto_select_first_disk(
    mut turn_state: ResMut<TurnState>,
    disks: Query<(Entity, &PlayerDisk), Without<TurnControlled>>,
    mut commands: Commands,
    mut sprites: Query<&mut Sprite>,
) {
    if turn_state.selected_entity.is_none() && !turn_state.in_motion {
        for (entity, disk) in &disks {
            if disk.player_id == turn_state.current_turn {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = Color::YELLOW;
                }
                commands.entity(entity).insert(TurnControlled);
                turn_state.selected_entity = Some(entity);
                break;
            }
        }
    }
}

pub fn cycle_disk_selection(
    keys: Res<Input<KeyCode>>,
    disks: Query<(Entity, &PlayerDisk), With<RigidBody>>,
    mut sprites: Query<&mut Sprite>,
    mut turn_state: ResMut<TurnState>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Tab) && !turn_state.in_motion {
        let mut player_disks: Vec<_> = disks
            .iter()
            .filter(|(_, d)| d.player_id == turn_state.current_turn)
            .collect();

        player_disks.sort_by_key(|(e, _)| e.index());

        if !player_disks.is_empty() {
            let current_index = turn_state.selected_entity.and_then(|current| {
                player_disks.iter().position(|(e, _)| *e == current)
            });

            if let Some(current) = turn_state.selected_entity {
                if let Ok(mut sprite) = sprites.get_mut(current) {
                    let color = player_disks
                        .iter()
                        .find(|(e, _)| *e == current)
                        .map(|(_, disk)| if disk.player_id == 1 { Color::BLUE } else { Color::GREEN })
                        .unwrap_or(Color::WHITE);
                    sprite.color = color;
                }
                commands.entity(current).remove::<TurnControlled>();
            }

            let next_index = match current_index {
                Some(i) => (i + 1) % player_disks.len(),
                None => 0,
            };

            let (new_entity, _) = player_disks[next_index];
            if let Ok(mut sprite) = sprites.get_mut(new_entity) {
                sprite.color = Color::YELLOW;
            }
            commands.entity(new_entity).insert(TurnControlled);
            turn_state.selected_entity = Some(new_entity);
            turn_state.aim_direction = Vec2::ZERO;
            turn_state.power = 0.0;
        }
    }
}

pub fn aim_with_keyboard(
    keys: Res<Input<KeyCode>>,
    mut turn_state: ResMut<TurnState>,
) {
    let mut direction = turn_state.aim_direction;
    if keys.pressed(KeyCode::Left) {
        direction.x -= 0.1;
    }
    if keys.pressed(KeyCode::Right) {
        direction.x += 0.1;
    }
    if keys.pressed(KeyCode::Up) {
        direction.y += 0.1;
    }
    if keys.pressed(KeyCode::Down) {
        direction.y -= 0.1;
    }
    turn_state.aim_direction = direction.clamp_length_max(1.0);
}

pub fn charge_shot_power(
    keys: Res<Input<KeyCode>>,
    mut turn_state: ResMut<TurnState>,
) {
    if keys.pressed(KeyCode::Space) {
        turn_state.power = (turn_state.power + 0.02).min(1.0);
    }
}

pub fn fire_selected_disk(
    keys: Res<Input<KeyCode>>,
    mut turn_state: ResMut<TurnState>,
    mut velocities: Query<(Entity, &mut Velocity), With<TurnControlled>>,
    mut commands: Commands,
) {
    if keys.just_released(KeyCode::Space) && !turn_state.in_motion {
        let direction = turn_state.aim_direction.normalize_or_zero();
        let speed = turn_state.power * 800.0;
        let force = direction * speed;

        println!("VELOCIDAD APLICADA: {:?}", force);

        let mut applied = false;

        for (entity, mut velocity) in &mut velocities {
            velocity.linvel = force;
            commands.entity(entity).remove::<Sleeping>();
            println!("\t-> Velocidad aplicada a entidad {:?}", entity);
            applied = true;
        }

        if applied {
            turn_state.in_motion = true;
            turn_state.power = 0.0;
        } else {
            println!("\t-> No se aplicó velocidad: no hay entidad con TurnControlled");
        }
    }
}

pub fn check_turn_end(
    mut turn_state: ResMut<TurnState>,
    velocities: Query<&Velocity, With<RigidBody>>,
    mut commands: Commands,
    entities: Query<Entity, With<TurnControlled>>,
    mut sprites: Query<&mut Sprite>,
    disks: Query<&PlayerDisk>,
) {
    if !turn_state.in_motion {
        return;
    }

    let threshold = 0.1;
    let all_stopped = velocities.iter().all(|v| v.linvel.length_squared() < threshold);

    if all_stopped {
        turn_state.in_motion = false;
        for entity in &entities {
            if let Ok(disk) = disks.get(entity) {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = if disk.player_id == 1 { Color::BLUE } else { Color::GREEN };
                }
            }
            commands.entity(entity).remove::<TurnControlled>();
        }
        turn_state.selected_entity = None;
        turn_state.current_turn = turn_state.current_turn % 2 + 1;
    }
}

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
    mut transforms: Query<(&mut Transform, Option<&PlayerDisk>, Option<&Ball>)>,
    mut sprites: Query<&mut Sprite>,
    mut commands: Commands,
) {
    for event in goal_events.read() {
        if event.scored_by_left {
            scores.left += 1;
            println!("Gol para el jugador izquierdo! Puntos: {}", scores.left);
        } else {
            scores.right += 1;
            println!("Gol para el jugador derecho! Puntos: {}", scores.right);
        }

        for (mut transform, is_disk, is_ball) in &mut transforms {
            if is_ball.is_some() {
                transform.translation = Vec3::ZERO;
            } else if let Some(disk) = is_disk {
                let positions = if disk.player_id == 1 {
                    [
                        Vec2::new(-250.0, 0.0),
                        Vec2::new(-150.0, 100.0),
                        Vec2::new(-150.0, -100.0),
                        Vec2::new(-50.0, 50.0),
                        Vec2::new(-50.0, -50.0),
                    ]
                } else {
                    [
                        Vec2::new(250.0, 0.0),
                        Vec2::new(150.0, 100.0),
                        Vec2::new(150.0, -100.0),
                        Vec2::new(50.0, 50.0),
                        Vec2::new(50.0, -50.0),
                    ]
                };
                let idx = transform.translation.y as i32 / 100 + 2;
                let pos = positions.get(idx as usize % positions.len()).unwrap_or(&positions[0]);
                transform.translation = pos.extend(0.0);
            }
        }

        if let Some(entity) = turn_state.selected_entity {
            if let Ok(mut sprite) = sprites.get_mut(entity) {
                sprite.color = if turn_state.current_turn == 1 {
                    Color::BLUE
                } else {
                    Color::GREEN
                };
            }
            commands.entity(entity).remove::<TurnControlled>();
        }

        turn_state.in_motion = false;
        turn_state.selected_entity = None;
        turn_state.aim_direction = Vec2::ZERO;
        turn_state.power = 0.0;
        turn_state.current_turn = turn_state.current_turn % 2 + 1;
    }
}

pub fn update_turn_text(turn_state: Res<TurnState>, mut query: Query<&mut Text, With<TurnText>>) {
    if turn_state.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Turno: Jugador {}", turn_state.current_turn);
    }
}

pub fn update_score_text(scores: Res<Scores>, mut texts: Query<&mut Text, With<ScoreText>>) {
    if scores.is_changed() {
        let mut text = texts.single_mut();
        text.sections[0].value = format!("P1: {}  -  P2: {}", scores.left, scores.right);
    }
}
pub fn draw_aim_direction_gizmo(
    mut gizmos: Gizmos,
    turn_state: Res<TurnState>,
    query: Query<&Transform, With<TurnControlled>>,
) {
    if let Some(entity) = turn_state.selected_entity {
        if let Ok(transform) = query.get(entity) {
            let start = transform.translation.truncate();
            let end = start + turn_state.aim_direction * 100.0;
            gizmos.line_2d(start, end, Color::YELLOW);
        }
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

pub fn animate_selected_disk(
    time: Res<Time>,
    turn_state: Res<TurnState>,
    mut query: Query<&mut Sprite>,
) {
    if let Some(selected) = turn_state.selected_entity {
        if let Ok(mut sprite) = query.get_mut(selected) {
            let t = (time.elapsed_seconds() * 6.0).sin() * 0.5 + 0.5;
            let mut color = sprite.color;
            color.set_a(0.2 + 0.8 * t); // nueva transparencia
            sprite.color = color; // reasignar el color entero
        }
    }
}
