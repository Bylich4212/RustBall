use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod events;
mod setup;
mod systems;

use resources::*;
use events::*;
use setup::setup;
use systems::{
    auto_select_first_disk,
    cycle_disk_selection,
    aim_with_keyboard,
    charge_shot_power,
    fire_selected_disk,
    check_turn_end,
    detect_goal,
    handle_goal,
    update_turn_text,
    update_score_text, // ✅ agregado
    draw_aim_direction_gizmo,
    update_power_bar,//nuevo
    animate_selected_disk, //nuevo
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.7, 0.3)))
        .insert_resource(TurnState {
            current_turn: 1,
            selected_entity: None,
            aim_direction: Vec2::ZERO,
            power: 0.0,
            in_motion: false,
        })
        .insert_resource(Scores::default())
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_event::<GoalEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            auto_select_first_disk,
            cycle_disk_selection,
            aim_with_keyboard,
            charge_shot_power,
            check_turn_end,
            detect_goal,
            handle_goal,
            update_turn_text,
            update_score_text, // ✅ agregado aquí
            animate_selected_disk, //nuevo
        ))
        .add_systems(PostUpdate, (
            fire_selected_disk, // ✅ lo mantenemos en PostUpdate
            draw_aim_direction_gizmo, //nuevo
            update_power_bar,//nuevo
        ))
        .run();
}
