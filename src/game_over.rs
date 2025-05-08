use bevy::prelude::*;
use crate::resources::Scores;

#[derive(Component)]
pub struct GameOverUI;

pub fn show_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scores: Res<Scores>,
) {
    // Asegurar cámara
    commands.spawn(Camera2dBundle::default());

    let (winner_text, final_score) = if scores.left > scores.right {
        ("¡Ganador: Jugador Izquierdo!", format!("{} - {}", scores.left, scores.right))
    } else {
        ("¡Ganador: Jugador Derecho!", format!("{} - {}", scores.right, scores.left))
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 0.2).into(),
            ..default()
        },
        GameOverUI,
    ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                winner_text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }));

            parent.spawn(TextBundle::from_section(
                format!("Marcador final: {}", final_score),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::GOLD,
                },
            ));
        });
}

pub fn cleanup_game_over_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<GameOverUI>>,
    cameras: Query<Entity, With<Camera>>,
) {
    for entity in &ui_elements {
        commands.entity(entity).despawn_recursive();
    }
    for cam in &cameras {
        commands.entity(cam).despawn_recursive(); // Limpia la cámara de GameOver
    }
}
