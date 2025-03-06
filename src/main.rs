use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use arrow::ArrowPlugin;
use gamestate::{GameState, GameStatePlugin};
use maze::MazePlugin;
use node::NodePlugin;
use path::PathPlugin;

mod arrow;
mod gamestate;
mod maze;
mod node;
mod path;

const MAZE_WIDTH: f32 = 15.0;
const MAZE_HEIGHT: f32 = 15.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Assasin".to_string(),
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
        ))
        .insert_resource(MazeUpdateTimer(Timer::from_seconds(
            0.00001,
            TimerMode::Repeating,
        )))
        .insert_resource(MazeShape(Vec2::new(MAZE_WIDTH, MAZE_HEIGHT)))
        .add_systems(Startup, setup)
        .add_plugins(GameStatePlugin)
        .add_plugins(MazePlugin {
            state: GameState::Running,
        })
        .add_plugins((
            NodePlugin {
                state: GameState::Running,
            },
            ArrowPlugin {
                state: GameState::Running,
            },
        ))
        .add_plugins(PathPlugin {
            state: GameState::Running,
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct MazeShape(pub Vec2);

#[derive(Resource)]
pub struct MazeUpdateTimer(pub Timer);
