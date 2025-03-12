use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use camera::CameraPlugin;
use iyes_perf_ui::prelude::*;

use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use fog::FogPlugin;
use gamestate::{GameState, GameStatePlugin};
use maze::MazePlugin;
use maze_specs::{MazeColor, MazeShape};
use player::PlayerPlugin;
use walls::WallPlugin;

mod camera;
mod fog;
mod gamestate;
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
            RapierDebugRenderPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(MazeColor {
            path_color: Color::srgb(0.2, 0.2, 0.2),
            wall_color: Color::srgb(0.8, 0.8, 0.8),
            root_color: Color::srgb(1.0, 0.0, 0.0),
            node_color: Color::srgb(0.0, 1.0, 0.0),
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
            state: GameState::Running,
        })
        .add_plugins(WallPlugin {
            state: GameState::Running,
        })
        .add_plugins(PlayerPlugin)
        .add_plugins(FogPlugin {
            state: GameState::Running,
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            values_col_width: 32.0,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}

#[derive(Resource)]
pub struct MazeUpdateTimer(pub Timer);
