use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
};

pub struct WallPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for WallPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_walls);
        app.add_systems(
            Update,
            (spawn_colliders, remove_colliders)
                .chain()
                .run_if(in_state(self.state.clone())),
        );
    }
}

#[derive(Component)]
struct PathCollider;

fn setup_walls(
    maze: Res<Maze>,
    mut commands: Commands,
    node_query: Query<&MazeNode>,
    color: Res<MazeColor>,
) {
    // Background
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.grid[0].len() as f32 * maze.cell_size,
                    maze.grid.len() as f32 * maze.cell_size,
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
            ..default()
        },
        Fill::color(color.path_color),
    ));

    // Upper border
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.grid[0].len() as f32 * maze.cell_size
                        + (maze.cell_size - maze.path_thickness),
                    maze.cell_size - maze.path_thickness,
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(
                0.,
                maze.grid.len() as f32 * maze.cell_size * 0.5,
                0.,
            )),
            ..default()
        },
        Fill::color(color.wall_color),
        Collider::cuboid(
            maze.grid[0].len() as f32 * maze.cell_size * 0.5
                + (maze.cell_size - maze.path_thickness) * 0.5,
            (maze.cell_size - maze.path_thickness) * 0.5,
        ),
    ));

    // Lower border
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.grid[0].len() as f32 * maze.cell_size
                        + (maze.cell_size - maze.path_thickness),
                    maze.cell_size - maze.path_thickness,
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(
                0.,
                maze.grid.len() as f32 * maze.cell_size * -0.5,
                0.,
            )),
            ..default()
        },
        Fill::color(color.wall_color),
        Collider::cuboid(
            maze.grid[0].len() as f32 * maze.cell_size * 0.5
                + (maze.cell_size - maze.path_thickness) * 0.5,
            (maze.cell_size - maze.path_thickness) * 0.5,
        ),
    ));

    // Right border
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.cell_size - maze.path_thickness,
                    maze.grid.len() as f32 * maze.cell_size
                        - (maze.cell_size - maze.path_thickness),
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(
                maze.grid[0].len() as f32 * maze.cell_size * 0.5,
                0.,
                0.,
            )),
            ..default()
        },
        Fill::color(color.wall_color),
        Collider::cuboid(
            (maze.cell_size - maze.path_thickness) * 0.5,
            maze.grid.len() as f32 * maze.cell_size * 0.5
                - (maze.cell_size - maze.path_thickness) * 0.5,
        ),
    ));

    // Left border
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.cell_size - maze.path_thickness,
                    maze.grid.len() as f32 * maze.cell_size
                        - (maze.cell_size - maze.path_thickness),
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(
                maze.grid[0].len() as f32 * maze.cell_size * -0.5,
                0.,
                0.,
            )),
            ..default()
        },
        Fill::color(color.wall_color),
        Collider::cuboid(
            (maze.cell_size - maze.path_thickness) * 0.5,
            maze.grid.len() as f32 * maze.cell_size * 0.5
                - (maze.cell_size - maze.path_thickness) * 0.5,
        ),
    ));

    for node in node_query.iter() {
        if node.position.x == maze.grid[0].len() as f32 - 1. {
            continue;
        }
        if node.position.y == maze.grid.len() as f32 - 1. {
            continue;
        }

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(
                        maze.cell_size - maze.path_thickness,
                        maze.cell_size - maze.path_thickness,
                    ),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    node.position.x * maze.cell_size
                        - maze.grid[0].len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size,
                    node.position.y * maze.cell_size
                        - maze.grid.len() as f32 * maze.cell_size * 0.5
                        + maze.cell_size,
                    0.,
                )),
                ..default()
            },
            Fill::color(color.wall_color),
            Collider::cuboid(
                (maze.cell_size - maze.path_thickness) * 0.5,
                (maze.cell_size - maze.path_thickness) * 0.5,
            ),
        ));
    }
}

fn spawn_colliders(
    path_colliders: Query<Entity, With<PathCollider>>,
    maze: Res<Maze>,
    mut commands: Commands,
    node_query: Query<&MazeNode>,
    color: Res<MazeColor>,
) {
    for entity in path_colliders.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn colliders for all the walls
    for node in node_query.iter() {
        let mut directions = Vec::new();

        if node.position.x > 0. {
            directions.push(Vec2::new(-1., 0.));
        }
        if node.position.x < maze.grid[0].len() as f32 - 1. {
            directions.push(Vec2::new(1., 0.));
        }
        if node.position.y > 0. {
            directions.push(Vec2::new(0., -1.));
        }
        if node.position.y < maze.grid.len() as f32 - 1. {
            directions.push(Vec2::new(0., 1.));
        }

        directions.iter().for_each(|direction| {
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
            let shape = shapes::Rectangle {
                extents: Vec2::new(
                    (maze.cell_size - maze.path_thickness) * direction.x
                        + maze.path_thickness * direction.y,
                    (maze.cell_size - maze.path_thickness) * direction.y
                        + maze.path_thickness * direction.x,
                ),
                ..default()
            };

            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_translation(Vec3::new(
                        node.position.x * maze.cell_size
                            - maze.grid[0].len() as f32 * maze.cell_size * 0.5
                            + maze.cell_size * num_x,
                        node.position.y * maze.cell_size
                            - maze.grid.len() as f32 * maze.cell_size * 0.5
                            + maze.cell_size * num_y,
                        0.,
                    )),
                    ..default()
                },
                Fill::color(color.wall_color),
                Collider::cuboid(
                    ((maze.cell_size - maze.path_thickness) * direction.x
                        + maze.path_thickness * direction.y)
                        * 0.5,
                    ((maze.cell_size - maze.path_thickness) * direction.y
                        + maze.path_thickness * direction.x)
                        * 0.5,
                ),
                PathCollider,
            ));
        });
    }
}

fn remove_colliders(
    mut commands: Commands,
    path_colliders: Query<(Entity, &Transform), With<PathCollider>>,
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
