use bevy::{core_pipeline::bloom::Bloom, prelude::*};

use crate::{gamestate::GameState, player::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras);
        app.add_systems(Update, (follow_player).run_if(in_state(GameState::InGame)));
        app.add_systems(Update, center_camera.run_if(in_state(GameState::Scanning)));
        app.add_systems(Update, zoom_camera);
    }
}

fn setup_cameras(mut commands: Commands) {
    // Spawn a 2D camera for the Scanning state
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::NATURAL,
        OrthographicProjection::default_2d(),
    ));
}

const CAMERA_DECAY_RATE: f32 = 5.0;
const CAMERA_ZOOM_RATE: f32 = 5.0;

fn follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn zoom_camera(
    mut projection: Query<&mut OrthographicProjection, (With<Camera2d>, Without<Player>)>,
    game_state: Res<State<GameState>>,
    time: Res<Time>,
) {
    let zoom_target = match game_state.get() {
        GameState::InGame => 1.,
        _ => 2.,
    };

    let Ok(mut projection) = projection.get_single_mut() else {
        return;
    };

    let current_zoom = projection.scale;
    let zoom_amount = zoom_target - current_zoom;

    projection
        .scale
        .smooth_nudge(&zoom_amount, CAMERA_ZOOM_RATE, time.delta_secs());
}

fn center_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let direction = Vec3::new(0., 0., camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
