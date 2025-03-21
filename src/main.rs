use bevy::prelude::*;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_prototype_lyon::prelude::*;
use camera::CameraPlugin;
use iyes_perf_ui::{
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
    prelude::*,
};

use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use gamestate::{GameState, GameStatePlugin};
use maze::MazePlugin;
use maze_specs::{MazeColor, MazeShape};
use player::PlayerPlugin;
use walls::WallPlugin;

mod camera;
mod gamestate;
mod hud;
mod maze;
mod maze_specs;
mod player;
mod walls;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Assassin".to_string(),
                        resolution: Vec2::new(512., 512.).into(),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        canvas: Some("#bevy".to_string()),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            ShapePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            // RapierDebugRenderPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            Light2dPlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(MazeColor {
            path_color: Color::srgb(0.2, 0.2, 0.2),
            wall_color: Color::srgb(0.8, 0.8, 0.8),
            player_color: Color::srgb(0.0, 0.0, 1.0),
        })
        .insert_resource(MazeShape(Vec2::new(15., 15.)))
        .insert_resource(MazeUpdateTimer(Timer::from_seconds(
            0.0125,
            TimerMode::Repeating,
        )))
        .add_plugins(GameStatePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MazePlugin {
            state: GameState::InGame,
        })
        .add_plugins(WallPlugin {
            state: GameState::InGame,
        })
        .add_plugins(PlayerPlugin {
            state: GameState::InGame,
        })
        .add_plugins(hud::HudPlugin {
            state: GameState::InGame,
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        // Contains everything related to FPS and frame time
        PerfUiFramerateEntries::default(),
        // Contains everything related to the window and cursor
        PerfUiWindowEntries::default(),
        // Contains everything related to system diagnostics (CPU, RAM)
        PerfUiSystemEntries::default(),
        // Contains everything related to fixed timestep
        PerfUiFixedTimeEntries::default(),
        // ...
    ));
}

#[derive(Resource)]
pub struct MazeUpdateTimer(pub Timer);
