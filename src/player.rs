use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::maze::Maze;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, maze: Res<Maze>) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(maze.cell_size * 0.3, maze.cell_size * 0.3),
        origin: RectangleOrigin::Center,
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
            ..default()
        },
        Fill::color(Color::srgb(1., 0., 0.)),
        Player,
    ));
}
