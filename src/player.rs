use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

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
    commands.spawn((
        RigidBody::Dynamic,
        Transform::from_xyz(0., 0., 10.),
        Velocity {
            linvel: Vec2::new(0., 10.),
            angvel: 0.2,
        },
        GravityScale(1.),
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(maze.cell_size * 0.2),
        ColliderMassProperties::Density(2.0),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0., 0., 0.),
        Collider::cuboid(maze.cell_size, maze.cell_size),
    ));
}
