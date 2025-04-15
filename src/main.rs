use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.3, 0.7, 0.3))) // Fondo verde
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, player2_movement, circle_reaction))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct ReactiveCircle;

#[derive(Component)]
struct GoalPost;

fn setup(mut commands: Commands) {
    // Cámara
    commands.spawn(Camera2dBundle::default());
    
    // Límites de la pantalla (para posicionar los arcos)
    let window_width = 800.0; // Ancho asumido de la ventana
    
    // Nivel del "suelo" - todos los elementos se alinearán en esta coordenada Y
    let ground_level = 0.0;
    
    // Jugador 1
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(-300.0, ground_level, 0.0),
            ..default()
        },
        Player,
    ));
    
    // Jugador 2
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.8, 0.2), // Color verde para distinguirlo
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, ground_level, 0.0),
            ..default()
        },
        Player2,
    ));
    
    // Círculo reactivo (ahora blanco en lugar de rojo)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0), // Blanco
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, ground_level, 0.0),
            ..default()
        },
        ReactiveCircle,
    ));
    
    // Arco izquierdo
    spawn_goal(&mut commands, -window_width/2.0 + 50.0, ground_level);
    
    // Arco derecho
    spawn_goal(&mut commands, window_width/2.0 - 50.0, ground_level);
}

fn spawn_goal(commands: &mut Commands, x: f32, y: f32) {
    // Poste vertical principal (ahora solo la parte superior del arco)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y + 50.0, 0.0),
            ..default()
        },
        GoalPost,
    ));
    
    // Poste horizontal superior (travesaño)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(60.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(x + (if x < 0.0 { 25.0 } else { -25.0 }), y + 100.0, 0.0),
            ..default()
        },
        GoalPost,
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
        
        // Solo permitir movimiento en el eje X
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

fn player2_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player2>>,
    time: Res<Time>,
) {
    let speed = 200.0;
    
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        
        // Solo permitir movimiento en el eje X
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

fn circle_reaction(
    player_query: Query<&Transform, With<Player>>,
    player2_query: Query<&Transform, (With<Player2>, Without<Player>)>,
    mut circle_query: Query<&mut Transform, (With<ReactiveCircle>, Without<Player>, Without<Player2>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player2_transform = player2_query.single();
    let mut circle_transform = circle_query.single_mut();
    
    // La distancia a la que el círculo comenzará a reaccionar
    let reaction_distance = 100.0;
    
    // Reacción al jugador 1
    let distance_player1 = player_transform.translation.distance(circle_transform.translation);
    if distance_player1 < reaction_distance {
        // Calcular dirección de escape (alejándose del jugador)
        let direction = (circle_transform.translation - player_transform.translation).normalize();
        
        // La velocidad de escape aumenta cuando el jugador está más cerca
        let escape_speed = 150.0 * (1.0 - distance_player1 / reaction_distance);
        
        // Mover el círculo (solo en el eje X para mantenerlo en la línea)
        let mut escape_movement = direction * escape_speed * time.delta_seconds();
        escape_movement.y = 0.0; // Restricción para mantenerlo en la misma línea Y
        circle_transform.translation += escape_movement;
    }
    
    // Reacción al jugador 2
    let distance_player2 = player2_transform.translation.distance(circle_transform.translation);
    if distance_player2 < reaction_distance {
        // Calcular dirección de escape (alejándose del jugador 2)
        let direction = (circle_transform.translation - player2_transform.translation).normalize();
        
        // La velocidad de escape aumenta cuando el jugador está más cerca
        let escape_speed = 150.0 * (1.0 - distance_player2 / reaction_distance);
        
        // Mover el círculo (solo en el eje X para mantenerlo en la línea)
        let mut escape_movement = direction * escape_speed * time.delta_seconds();
        escape_movement.y = 0.0; // Restricción para mantenerlo en la misma línea Y
        circle_transform.translation += escape_movement;
    }
}