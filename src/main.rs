use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, circle_reaction))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct ReactiveCircle;

fn setup(mut commands: Commands) {
    // Cámara
    commands.spawn(Camera2dBundle::default());

    // Jugador
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
    ));

    // Círculo reactivo
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.3, 0.3),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(150.0, 150.0, 0.0),
            ..default()
        },
        ReactiveCircle,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 200.0;

    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

fn circle_reaction(
    player_query: Query<&Transform, With<Player>>,
    mut circle_query: Query<&mut Transform, (With<ReactiveCircle>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let mut circle_transform = circle_query.single_mut();
    
    // Calcular la distancia entre el jugador y el círculo
    let distance = player_transform.translation.distance(circle_transform.translation);
    
    // La distancia a la que el círculo comenzará a reaccionar
    let reaction_distance = 100.0;
    
    if distance < reaction_distance {
        // Calcular dirección de escape (alejándose del jugador)
        let direction = (circle_transform.translation - player_transform.translation).normalize();
        
        // La velocidad de escape aumenta cuando el jugador está más cerca
        let escape_speed = 150.0 * (1.0 - distance / reaction_distance);
        
        // Mover el círculo
        circle_transform.translation += direction * escape_speed * time.delta_seconds();
    }
}