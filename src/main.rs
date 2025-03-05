use bevy::prelude::*;
use maze::MazePlugin;
use walls::PathPlugin;

mod maze;
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
        .add_systems(Startup, setup)
        .add_plugins((MazePlugin, PathPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
