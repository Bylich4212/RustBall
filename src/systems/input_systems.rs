use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy_rapier2d::prelude::*;

use crate::resources::*;
use crate::components::*;

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
