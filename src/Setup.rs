use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Texto de turno
    commands.spawn((
        TextBundle::from_section(
            "Turno: Jugador 1",
            TextStyle {
                font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
        TurnText,
    ));

    // Texto de marcador
    commands.spawn((
        TextBundle::from_section(
            "Jugador 1: 0 | Jugador 2: 0",
            TextStyle {
                font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                left: Val::Px(10.0),
                ..default()
            }),
        ScoreText,
    ));

    // Barra de poder
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            ..default()
        },
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..default()
    })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(0.0), // Se ajustará dinámicamente
                        height: Val::Px(20.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::YELLOW),
                    ..default()
                },
                PowerBar,
            ));
        });


    let wall_thickness = 10.0;
    let bounds = Vec2::new(800.0, 600.0);
    let half_w = bounds.x / 2.0;
    let half_h = bounds.y / 2.0;

    let walls = [
        Vec3::new(-half_w, 0.0, 0.0),
        Vec3::new(half_w, 0.0, 0.0),
        Vec3::new(0.0, -half_h - wall_thickness / 2.0, 0.0),
        Vec3::new(0.0, half_h, 0.0),
    ];

    for pos in walls {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(
                    if pos.x == 0.0 { bounds.x } else { wall_thickness },
                    if pos.y == 0.0 { bounds.y } else { wall_thickness },
                )),
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        })
            .insert(Collider::cuboid(
                if pos.x == 0.0 { bounds.x / 2.0 } else { wall_thickness / 2.0 },
                if pos.y == 0.0 { bounds.y / 2.0 } else { wall_thickness / 2.0 },
            ))
            .insert(RigidBody::Fixed);
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(20.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(-half_w + 5.0, 0.0, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 60.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        GoalZone { is_left: true },
    ));


    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(20.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(half_w - 5.0, 0.0, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 60.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        GoalZone { is_left: false },
    ));


    let positions_p1 = vec![
        Vec2::new(-250.0, 0.0),
        Vec2::new(-150.0, 100.0),
        Vec2::new(-150.0, -100.0),
        Vec2::new(-50.0, 50.0),
        Vec2::new(-50.0, -50.0),
    ];

    let positions_p2 = vec![
        Vec2::new(250.0, 0.0),
        Vec2::new(150.0, 100.0),
        Vec2::new(150.0, -100.0),
        Vec2::new(50.0, 50.0),
        Vec2::new(50.0, -50.0),
    ];

    for pos in positions_p1 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("circulobarca.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(25.0),
            ActiveEvents::COLLISION_EVENTS,
            ExternalImpulse::default(),
            ExternalForce::default(),
            AdditionalMassProperties::Mass(1.0),
            Velocity::zero(),
            Damping {
                linear_damping: 0.8,
                angular_damping: 0.5,
            },
            LockedAxes::ROTATION_LOCKED,
            Sleeping::disabled(),
            PlayerDisk { player_id: 1 },
        ));
    }

    for pos in positions_p2 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("circuloparis.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(25.0),
            ActiveEvents::COLLISION_EVENTS,
            ExternalImpulse::default(),
            ExternalForce::default(),
            AdditionalMassProperties::Mass(1.0),
            Velocity::zero(),
            Damping {
                linear_damping: 0.8,
                angular_damping: 0.5,
            },
            LockedAxes::ROTATION_LOCKED,
            Sleeping::disabled(),
            PlayerDisk { player_id: 2 },
        ));
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pelota.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(15.0),
        ActiveEvents::COLLISION_EVENTS,
        ExternalImpulse::default(),
        ExternalForce::default(),
        AdditionalMassProperties::Mass(1.0),
        Velocity::zero(),
        Damping {
            linear_damping: 0.8,
            angular_damping: 0.5,
        },
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Ball,
    ));
}