use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

pub fn spawn_players(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let damping = Damping {
        linear_damping: 2.0,
        angular_damping: 2.0,
    };

    let positions_p1 = vec![
        Vec2::new(-400.0, 0.0),
        Vec2::new(-300.0, 150.0),
        Vec2::new(-300.0, -150.0),
        Vec2::new(-150.0, 100.0),
        Vec2::new(-150.0, -100.0),
    ];

    let positions_p2 = vec![
        Vec2::new(400.0, 0.0),
        Vec2::new(300.0, 150.0),
        Vec2::new(300.0, -150.0),
        Vec2::new(150.0, 100.0),
        Vec2::new(150.0, -100.0),
    ];

    for pos in positions_p1 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("circulobarca.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(35.0),
            Restitution::coefficient(0.5),
            ActiveEvents::COLLISION_EVENTS,
            ExternalImpulse::default(),
            ExternalForce::default(),
            AdditionalMassProperties::Mass(1.0),
            Velocity::zero(),
            damping.clone(),
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
                    custom_size: Some(Vec2::splat(70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(35.0),
            Restitution::coefficient(0.5),
            ActiveEvents::COLLISION_EVENTS,
            ExternalImpulse::default(),
            ExternalForce::default(),
            AdditionalMassProperties::Mass(1.0),
            Velocity::zero(),
            damping.clone(),
            LockedAxes::ROTATION_LOCKED,
            Sleeping::disabled(),
            PlayerDisk { player_id: 2 },
        ));
    }
}
