use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod events;
mod systems;
mod setup;
mod formation;
mod formation_selection;

use crate::resources::AppState;
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
    update_score_text,
    draw_aim_direction_gizmo,
    update_power_bar,
    animate_selected_disk,
    reset_for_formation,
};
use formation_selection::{handle_formation_click, cleanup_formation_ui};

/// 🔁 Adaptador para poder usar `show_formation_ui` desde `OnEnter` sin conflictos de ownership
fn show_formation_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    formation_selection::show_formation_ui(&mut commands, &asset_server);
}

fn main() {
    App::new()
        // Fondo
        .insert_resource(ClearColor(Color::BLACK))

        // Estados del juego
        .add_state::<AppState>()

        // Recursos iniciales
        .insert_resource(TurnState {
            current_turn: 1,
            selected_entity: None,
            aim_direction: Vec2::ZERO,
            power: 0.0,
            in_motion: false,
        })
        .insert_resource(Scores::default())
        .insert_resource(PlayerFormations {
            player1: None,
            player2: None,
        })

        // Plugins
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })

        // Eventos
        .add_event::<GoalEvent>()

        // UI de formación
        .add_systems(OnEnter(AppState::FormationSelection), show_formation_ui_system)
        .add_systems(OnEnter(AppState::FormationChange), show_formation_ui_system)

        // Manejo de clicks para elegir formaciones
        .add_systems(Update, handle_formation_click.run_if(
            in_state(AppState::FormationSelection).or_else(in_state(AppState::FormationChange))
        ))

        // Cleanup al pasar a InGame
        .add_systems(OnEnter(AppState::InGame), cleanup_formation_ui)

        // Setup del juego
        .add_systems(OnEnter(AppState::InGame), setup)

        // Reinicio tras gol
        .add_systems(OnEnter(AppState::FormationChange), reset_for_formation)

        // Lógica activa del juego
        .add_systems(Update, (
            auto_select_first_disk,
            cycle_disk_selection,
            aim_with_keyboard,
            charge_shot_power,
            check_turn_end,
            detect_goal,
            handle_goal,
            update_turn_text,
            update_score_text,
            animate_selected_disk,
        ).run_if(in_state(AppState::InGame)))

        // Post-update
        .add_systems(PostUpdate, (
            fire_selected_disk,
            draw_aim_direction_gizmo,
            update_power_bar,
        ).run_if(in_state(AppState::InGame)))

        // Inicia juego
        .run();
}
