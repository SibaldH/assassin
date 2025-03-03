use bevy::prelude::*;

mod maze;
mod resolution;
mod wall;

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
        .add_plugins((resolution::ResolutionPlugin, maze::MazePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub const MAZE: &str = "
1 1 1 1 1 1 1
1 0 0 0 0 0 1
1 1 1 0 1 1 1
1 0 0 0 0 0 1
1 1 1 1 1 0 1
1 0 0 0 0 0 1
1 1 1 1 1 1 1
";
