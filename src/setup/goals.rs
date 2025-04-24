use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

pub fn spawn_goals(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // 📏 Dimensiones generales
    let goal_height = 200.0;
    let goal_width = 100.0;
    let wall_thickness = 10.0;
    let field_width = 1100.0;
    let half_field = field_width / 2.0;

    // 🧮 Medidas auxiliares
    let half_w = goal_width / 2.0;
    let half_h = goal_height / 2.0;
    let z_sensor = 0.0;
    let z_struct = 0.1;

    // ========================= 🥅 ARCO IZQUIERDO =========================
    let x_izq = -half_field - 10.0;

    // 🎯 Sensor de gol
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("arcoizq.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(goal_width, goal_height)),
                ..default()
            },
            transform: Transform::from_xyz(x_izq, 0.0, z_sensor),
            ..default()
        },
        Collider::cuboid(half_w-35.0, half_h-70.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        GoalZone { is_left: true },
    ));

    // 🚧 Poste izquierdo
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(20.0, goal_height)),
                ..default()
            },
            transform: Transform::from_xyz(x_izq+12.0 - half_w, 0.0, z_struct),
            ..default()
        },
        Collider::cuboid(wall_thickness / 2.0, half_h),
        RigidBody::Fixed,
    ));

    // 🚧 Travesaño superior
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(goal_width, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(x_izq-35.0, half_h, z_struct),
            ..default()
        },
        Collider::cuboid(half_w, wall_thickness / 2.0),
        RigidBody::Fixed,
    ));

    // 🚧 Piso inferior
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(goal_width, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(x_izq-35.0, -half_h, z_struct),
            ..default()
        },
        Collider::cuboid(half_w, wall_thickness / 2.0),
        RigidBody::Fixed,
    ));

    // ========================= 🥅 ARCO DERECHO =========================
    let x_der = half_field + 10.0;

    // 🎯 Sensor de gol
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("arcoder.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(goal_width, goal_height)),
                ..default()
            },
            transform: Transform::from_xyz(x_der, 0.0, z_sensor),
            ..default()
        },
        Collider::cuboid(half_w-35.0, half_h-70.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        GoalZone { is_left: false },
    ));

    // 🚧 Poste derecho (solamente este)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(20.0, goal_height)),
                ..default()
            },
            transform: Transform::from_xyz(x_der-12.0 + half_w, 0.0, z_struct),
            ..default()
        },
        Collider::cuboid(wall_thickness / 2.0, half_h),
        RigidBody::Fixed,
    ));

    // 🚧 Travesaño superior
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(goal_width, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(x_der+35.0, half_h, z_struct),
            ..default()
        },
        Collider::cuboid(half_w, wall_thickness / 2.0),
        RigidBody::Fixed,
    ));

    // 🚧 Piso inferior
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(goal_width, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(x_der+35.0, -half_h, z_struct),
            ..default()
        },
        Collider::cuboid(half_w, wall_thickness / 2.0),
        RigidBody::Fixed,
    ));
}
