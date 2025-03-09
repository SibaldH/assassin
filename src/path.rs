use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
    MazeUpdateTimer,
};

pub struct PathPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PathPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_path, setup_colliders));
        app.add_systems(Update, update_paths.run_if(in_state(self.state.clone())));
        app.add_systems(
            Update,
            (spawn_colliders, remove_colliders)
                .chain()
                .run_if(in_state(self.state.clone())),
        );
    }
}

const RADIUS_RATIO: f32 = 8.0;

#[derive(Component)]
struct SidePath;

#[derive(Component)]
struct Wall;

#[derive(Component, Debug)]
pub struct Path;

fn setup_path(
    mut commands: Commands,
    maze: Res<Maze>,
    color: Res<MazeColor>,
    node_query: Query<&MazeNode>,
) {
    // draw a border around the maze
    let points: Vec<Vec2> = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., maze.grid.len() as f32 * maze.cell_size),
        Vec2::new(
            maze.grid[0].len() as f32 * maze.cell_size,
            maze.grid.len() as f32 * maze.cell_size,
        ),
        Vec2::new(maze.grid[0].len() as f32 * maze.cell_size, 0.),
    ];

    let shape = shapes::RoundedPolygon {
        points,
        radius: maze.cell_size / RADIUS_RATIO,
        closed: false,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_translation(Vec3::new(
                maze.grid[0].len() as f32 * -maze.cell_size * 0.5,
                maze.grid.len() as f32 * -maze.cell_size * 0.5,
                -10.,
            )),
            ..default()
        },
        Fill::color(color.wall_color),
    ));

    for node in node_query.iter() {
        let node_translation = Vec3::new(
            node.position.x * maze.cell_size
                + maze.grid[0].len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            node.position.y * maze.cell_size
                + maze.grid.len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            0.,
        );

        let shape = shapes::Rectangle {
            extents: Vec2::splat(maze.path_thickness),
            radii: Some(BorderRadii::single(maze.path_thickness / 4.0)),
            origin: RectangleOrigin::Center,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(node_translation),
                ..default()
            },
            Fill::color(color.path_color),
            Path,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn update_paths(
    mut commands: Commands,
    node_query: Query<&MazeNode>,
    sprite_query: Query<Entity, With<SidePath>>,
    maze: Res<Maze>,
    color: Res<MazeColor>,
    time: Res<Time>,
    mut timer: ResMut<MazeUpdateTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // Despawn all existing sidepaths
    for path_entity in sprite_query.iter() {
        commands.entity(path_entity).despawn();
    }

    for node in node_query.iter() {
        if node.parent.is_none() {
            continue;
        }
        let parent_node = node_query.get(node.parent.unwrap()).unwrap();
        let parent_direction = parent_node.position - node.position;

        let node_translation = Vec3::new(
            node.position.x * maze.cell_size
                + maze.grid[0].len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            node.position.y * maze.cell_size
                + maze.grid.len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            0.,
        );

        let offset = Vec2::new(
            parent_direction.x * maze.cell_size * 0.5,
            parent_direction.y * maze.cell_size * 0.5,
        );

        let shape = shapes::Rectangle {
            extents: Vec2::splat(maze.path_thickness),
            radii: Some(BorderRadii::single(maze.path_thickness / 4.0)),
            origin: RectangleOrigin::Center,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    node_translation + Vec3::new(offset.x, offset.y, 0.),
                ),
                ..default()
            },
            Fill::color(color.path_color),
            SidePath,
            Path,
        ));
    }
}

fn setup_colliders(maze: Res<Maze>, mut commands: Commands, node_query: Query<&MazeNode>) {
    commands.spawn((
        Collider::cuboid(maze.grid[0].len() as f32 * maze.cell_size * 0.5, 1.),
        Transform::from_translation(Vec3::new(
            0.,
            maze.grid.len() as f32 * maze.cell_size * 0.5 + 1.
                - (maze.cell_size - maze.path_thickness) * 0.5,
            0.,
        )),
        RigidBody::Fixed,
    ));
    commands.spawn((
        Collider::cuboid(maze.grid[0].len() as f32 * maze.cell_size * 0.5, 1.),
        Transform::from_translation(Vec3::new(
            0.,
            maze.grid.len() as f32 * maze.cell_size * -0.5 - 1.
                + (maze.cell_size - maze.path_thickness) * 0.5,
            0.,
        )),
    ));
    commands.spawn((
        Collider::cuboid(1., maze.grid.len() as f32 * maze.cell_size * 0.5),
        Transform::from_translation(Vec3::new(
            maze.grid[0].len() as f32 * maze.cell_size * 0.5 + 1.
                - (maze.cell_size - maze.path_thickness) * 0.5,
            0.,
            0.,
        )),
    ));
    commands.spawn((
        Collider::cuboid(1., maze.grid.len() as f32 * maze.cell_size * 0.5),
        Transform::from_translation(Vec3::new(
            maze.grid[0].len() as f32 * maze.cell_size * -0.5 - 1.
                + (maze.cell_size - maze.path_thickness) * 0.5,
            0.,
            0.,
        )),
    ));

    for node in node_query.iter() {
        commands.spawn((
            Collider::cuboid(
                (maze.cell_size - maze.path_thickness) * 0.5,
                (maze.cell_size - maze.path_thickness) * 0.5,
            ),
            Transform::from_translation(Vec3::new(
                node.position.x * maze.cell_size - maze.grid[0].len() as f32 * maze.cell_size * 0.5
                    + maze.cell_size,
                node.position.y * maze.cell_size - maze.grid.len() as f32 * maze.cell_size * 0.5
                    + maze.cell_size,
                0.,
            )),
        ));
        if node.position.x == 0. {
            commands.spawn((
                Collider::cuboid(
                    (maze.cell_size - maze.path_thickness) * 0.5,
                    (maze.cell_size - maze.path_thickness) * 0.5,
                ),
                Transform::from_translation(Vec3::new(
                    node.position.x * maze.cell_size
                        - maze.grid[0].len() as f32 * maze.cell_size * 0.5,
                    node.position.y * maze.cell_size
                        - maze.grid.len() as f32 * maze.cell_size * 0.5,
                    0.,
                )),
            ));
        }
        if node.position.y == 0. {
            commands.spawn((
                Collider::cuboid(
                    (maze.cell_size - maze.path_thickness) * 0.5,
                    (maze.cell_size - maze.path_thickness) * 0.5,
                ),
                Transform::from_translation(Vec3::new(
                    node.position.x * maze.cell_size
                        - maze.grid[0].len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size,
                    node.position.y * maze.cell_size
                        - maze.grid.len() as f32 * maze.cell_size * 0.5,
                    0.,
                )),
            ));
        }
        if node.position.x == 0. && node.position.y == maze.grid.len() as f32 - 1. {
            commands.spawn((
                Collider::cuboid(
                    (maze.cell_size - maze.path_thickness) * 0.5,
                    (maze.cell_size - maze.path_thickness) * 0.5,
                ),
                Transform::from_translation(Vec3::new(
                    node.position.x * maze.cell_size
                        - maze.grid[0].len() as f32 * maze.cell_size * 0.5,
                    node.position.y * maze.cell_size
                        - maze.grid.len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size,
                    0.,
                )),
            ));
        }
    }
}

fn spawn_colliders(
    path_colliders: Query<Entity, With<Wall>>,
    maze: Res<Maze>,
    mut commands: Commands,
    node_query: Query<&MazeNode>,
) {
    for entity in path_colliders.iter() {
        commands.entity(entity).despawn();
    }

    let direcitons = [
        Vec2::new(0., 1.),
        Vec2::new(0., -1.),
        Vec2::new(1., 0.),
        Vec2::new(-1., 0.),
    ];

    // Spawn colliders for all the walls
    for node in node_query.iter() {
        for direction in direcitons.iter() {
            let num_x = match direction.x {
                1. => 1.,
                -1. => 0.,
                0. => 0.5,
                _ => 0.,
            };

            let num_y = match direction.y {
                1. => 1.,
                -1. => 0.,
                0. => 0.5,
                _ => 0.,
            };
            commands.spawn((
                Collider::cuboid(
                    ((maze.cell_size - maze.path_thickness) * direction.x
                        + maze.path_thickness * direction.y)
                        * 0.5,
                    ((maze.cell_size - maze.path_thickness) * direction.y
                        + maze.path_thickness * direction.x)
                        * 0.5,
                ),
                Transform::from_translation(Vec3::new(
                    node.position.x * maze.cell_size
                        - maze.grid[0].len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size * num_x,
                    node.position.y * maze.cell_size
                        - maze.grid.len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size * num_y,
                    0.,
                )),
                Wall,
            ));
        }
    }
}

fn remove_colliders(
    mut commands: Commands,
    path_colliders: Query<(Entity, &Transform), With<Wall>>,
    maze: Res<Maze>,
    node_query: Query<&MazeNode>,
) {
    for node in node_query.iter() {
        if node.parent.is_none() {
            continue;
        }
        let parent_node = node_query.get(node.parent.unwrap()).unwrap();
        let parent_direction = parent_node.position - node.position;

        let node_translation = Vec3::new(
            node.position.x * maze.cell_size
                + maze.grid[0].len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            node.position.y * maze.cell_size
                + maze.grid.len() as f32 * maze.cell_size * -0.5
                + maze.cell_size * 0.5,
            0.,
        );

        let offset = Vec2::new(
            parent_direction.x * maze.cell_size * 0.5,
            parent_direction.y * maze.cell_size * 0.5,
        );

        for (entity, transform) in path_colliders.iter() {
            if transform
                .translation
                .distance(node_translation + Vec3::new(offset.x, offset.y, 0.))
                < 0.1
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
