use bevy::prelude::*;
use bevy::audio::{PlaybackSettings, GlobalVolume};
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod events;
mod systems;
mod setup;
mod formation;
mod formation_selection;
mod game_over; // ✅ Agregado

use crate::resources::AppState;
use resources::*;
use events::*;
use setup::{setup, cleanup_cameras};
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
    animate_selection_buttons,
    goal_banner_fadeout,
    setup_goal_timer,
    wait_and_change_state,
};
use formation_selection::{handle_formation_click, cleanup_formation_ui};
use setup::ui::cleanup_power_bar;

// ✅ Importa funciones del módulo game_over
use game_over::{show_game_over_screen, cleanup_game_over_ui};

#[derive(Resource)]
struct TeamSelectionMusic(Handle<AudioSource>);
#[derive(Component)]
struct FormationMusicTag;

#[derive(Resource)]
struct InGameMusic(Handle<AudioSource>);
#[derive(Component)]
struct InGameMusicTag;

#[derive(Resource)]
struct GoalSound(Handle<AudioSource>);

#[derive(Resource)]
struct BackgroundImage(Handle<Image>);
#[derive(Component)]
struct BackgroundTag;

// 🎵 Música
fn load_team_selection_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu = asset_server.load("audio/uefa-champions-league-theme.mp3");
    let game = asset_server.load("audio/love_me_again.ogg");
    let goal = asset_server.load("audio/mariano-closs-ahi-estaaaaa-gooool.ogg");
    commands.insert_resource(TeamSelectionMusic(menu));
    commands.insert_resource(InGameMusic(game));
    commands.insert_resource(GoalSound(goal));
}

fn play_selection_music(music: Res<TeamSelectionMusic>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: music.0.clone(),
            settings: PlaybackSettings::LOOP,
        },
        FormationMusicTag,
    ));
}

fn stop_selection_music(mut commands: Commands, query: Query<Entity, With<FormationMusicTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_ingame_music(music: Res<InGameMusic>, mut commands: Commands) {
    println!("🎵 Iniciando música del juego...");
    commands.spawn((
        AudioBundle {
            source: music.0.clone(),
            settings: PlaybackSettings::LOOP,
        },
        InGameMusicTag,
    ));
}

fn stop_ingame_music(mut commands: Commands, query: Query<Entity, With<InGameMusicTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_goal_sound(audio: Res<GoalSound>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: audio.0.clone(),
        settings: PlaybackSettings::ONCE,
    });
}

// 🖼️ Fondo
fn load_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("championsfondo3.png");
    commands.insert_resource(BackgroundImage(image));
}

fn spawn_selection_background(mut commands: Commands, background: Res<BackgroundImage>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        },
        BackgroundTag,
    ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage::new(background.0.clone()),
                ..default()
            });
        });
}

fn despawn_selection_background(mut commands: Commands, query: Query<Entity, With<BackgroundTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_formation_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    formation_selection::show_formation_ui(&mut commands, &asset_server);
}

fn cleanup_cameras_on_enter(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    cleanup_cameras(&mut commands, query);
}

// 🏁 MAIN
fn main() {
    App::new()
        .insert_resource(GlobalVolume::new(1.0))
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>()
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
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_event::<GoalEvent>()

        // Startup
        .add_systems(Startup, (load_team_selection_music, load_background_image))

        // Menú
        .add_systems(OnEnter(AppState::FormationSelection), show_formation_ui_system)
        .add_systems(OnEnter(AppState::FormationChange), show_formation_ui_system)
        .add_systems(OnEnter(AppState::FormationSelection), spawn_selection_background)
        .add_systems(OnExit(AppState::FormationSelection), despawn_selection_background)
        .add_systems(OnEnter(AppState::FormationSelection), play_selection_music)
        .add_systems(OnExit(AppState::FormationSelection), stop_selection_music)

        // Interacción visual
        .add_systems(Update, (
            handle_formation_click,
            animate_selection_buttons,
        ).run_if(
            in_state(AppState::FormationSelection).or_else(in_state(AppState::FormationChange))
        ))

        // Setup de juego
        .add_systems(OnEnter(AppState::InGame), cleanup_formation_ui)
        .add_systems(OnEnter(AppState::InGame), cleanup_cameras_on_enter)
        .add_systems(OnEnter(AppState::InGame), play_ingame_music)
        .add_systems(OnExit(AppState::InGame), stop_ingame_music)
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnEnter(AppState::FormationChange), reset_for_formation)
        .add_systems(OnEnter(AppState::FormationChange), cleanup_power_bar)

        // Juego
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
        .add_systems(PostUpdate, (
            fire_selected_disk,
            draw_aim_direction_gizmo,
            update_power_bar,
        ).run_if(in_state(AppState::InGame)))

        // Gol
        .add_systems(OnEnter(AppState::GoalScored), setup_goal_timer)
        .add_systems(OnEnter(AppState::GoalScored), play_goal_sound)
        .add_systems(Update, (
            goal_banner_fadeout,
            wait_and_change_state,
        ).run_if(in_state(AppState::GoalScored)))

        // ✅ Game Over
        .add_systems(OnEnter(AppState::GameOver), show_game_over_screen)
        .add_systems(OnExit(AppState::GameOver), cleanup_game_over_ui)

        .run();
}
