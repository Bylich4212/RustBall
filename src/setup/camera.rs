use bevy::prelude::*;

pub fn spawn_camera_and_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Cámara
    commands.spawn(Camera2dBundle::default());

    // Fondo de cancha
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cancha.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -20.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..default()
    });
}
