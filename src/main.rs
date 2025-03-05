use bevy::prelude::*;

use arrow::ArrowPlugin;
use maze::MazePlugin;
use node::NodePlugin;
use walls::PathPlugin;

mod arrow;
mod maze;
mod node;
mod walls;

fn main() {
    App::new()
        .add_plugins(
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
        )
        .insert_resource(MazeUpdateTimer(Timer::from_seconds(
            0.25,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_plugins((MazePlugin, NodePlugin, ArrowPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct MazeUpdateTimer(pub Timer);
