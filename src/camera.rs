use bevy::prelude::*;

use crate::{gamestate::GameState, player::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras);
        app.add_systems(Update, follow_player.run_if(in_state(GameState::Running)));
        app.add_systems(Update, center_camera.run_if(in_state(GameState::Paused)));
    }
}

fn setup_cameras(mut commands: Commands) {
    // Spawn a 2D camera for the paused state
    commands.spawn(Camera2d);
}

const LERP_SPEED: f32 = 5.0;

fn follow_player(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation.truncate();

    let mut camera_transform = camera_query.single_mut();
    let new_pos = camera_transform.translation.lerp(
        player_pos.extend(0.0),
        1.0 - f32::exp(-LERP_SPEED * time.delta_secs()),
    );

    camera_transform.translation = new_pos;
}

fn center_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera_transform = camera_query.single_mut();

    let new_pos = camera_transform.translation.lerp(
        Vec3::new(0.0, 0.0, 0.0),
        1.0 - f32::exp(-LERP_SPEED * time.delta_secs()),
    );

    camera_transform.translation = new_pos;
}
