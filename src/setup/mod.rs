pub mod camera;
pub mod ui;
pub mod field;
pub mod players;
pub mod ball;
pub mod goals;

use bevy::prelude::*;

use camera::spawn_camera_and_background;
use ui::spawn_ui;
use field::spawn_walls;
use players::spawn_players;
use ball::spawn_ball;
use goals::spawn_goals;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_camera_and_background(&mut commands, &asset_server);
    spawn_ui(&mut commands, &asset_server);
    spawn_walls(&mut commands);
    spawn_players(&mut commands, &asset_server);
    spawn_ball(&mut commands, &asset_server);
    spawn_goals(&mut commands, &asset_server);
}
