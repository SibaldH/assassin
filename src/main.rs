use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use arrow::ArrowPlugin;
use maze::MazePlugin;
use node::NodePlugin;
use path::PathPlugin;

mod arrow;
mod maze;
mod node;
mod path;

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
        .add_systems(Startup, setup)
        .add_plugins(MazePlugin)
        // .add_plugins((NodePlugin, ArrowPlugin))
        .add_plugins(PathPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct MazeUpdateTimer(pub Timer);
