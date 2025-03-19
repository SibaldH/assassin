use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    gamestate::GameState,
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
    player::{Player, RangeNodes},
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
pub struct Wall;

fn setup_walls(maze: Res<Maze>, mut commands: Commands, color: Res<MazeColor>) {
    // Background
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(
                    maze.grid[0].len() as f32 * maze.cell_size
                        + (maze.cell_size - maze.path_thickness),
                    maze.grid.len() as f32 * maze.cell_size
                        + (maze.cell_size - maze.path_thickness),
                ),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
            ..default()
        },
        Fill::color(color.path_color),
    ));

    // Borders
    let directions = [
        Vec2::new(1., 0.),
        Vec2::new(0., 1.),
        Vec2::new(-1., 0.),
        Vec2::new(0., -1.),
    ];
    for direction in directions.iter() {
        let shape = shapes::Rectangle {
            extents: Vec2::new(
                maze.grid[0].len() as f32 * maze.cell_size * direction.x.abs()
                    + (maze.cell_size - maze.path_thickness),
                maze.grid.len() as f32 * maze.cell_size * direction.y.abs()
                    + (maze.cell_size - maze.path_thickness),
            ),
            ..default()
        };

        commands.spawn((
            // ShapeBundle {
            //     path: GeometryBuilder::build_as(&shape),
            //     transform:             //     ..default()
            // },
            // Fill::color(color.wall_color),
            Transform::from_translation(Vec3::new(
                maze.grid[0].len() as f32 * maze.cell_size * direction.y * 0.5,
                maze.grid.len() as f32 * maze.cell_size * direction.x * 0.5,
                0.,
            )),
            Collider::cuboid(shape.extents.x * 0.5, shape.extents.y * 0.5),
            LightOccluder2d {
                shape: LightOccluder2dShape::Rectangle {
                    half_size: Vec2::new(shape.extents.x * 0.5, shape.extents.y * 0.5),
                },
            },
        ));
    }
}

fn spawn_colliders(
    wall_colliders: Query<Entity, With<Wall>>,
    maze: Res<Maze>,
    mut commands: Commands,
    node_query: Query<(&MazeNode, Entity)>,
    range_nodes: Res<RangeNodes>,
    color: Res<MazeColor>,
    game_state: Res<State<GameState>>,
) {
    for entity in wall_colliders.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn colliders for all the walls
    for (node, node_entity) in node_query.iter() {
        //Check if the node is inside the range_nodes
        if !range_nodes.0.contains(&node_entity) && game_state.get() != &GameState::Paused {
            continue;
        }

        let mut directions = Vec::new();

        if node.index.x > 0. {
            directions.push(Vec2::new(-1., 0.));
        }
        if node.index.x < maze.grid[0].len() as f32 - 1. {
            directions.push(Vec2::new(1., 0.));
        }
        if node.index.y > 0. {
            directions.push(Vec2::new(0., -1.));
        }
        if node.index.y < maze.grid.len() as f32 - 1. {
            directions.push(Vec2::new(0., 1.));
        }

        directions.iter().for_each(|direction| {
            let shape = Vec2::new(
                (maze.cell_size - maze.path_thickness) * direction.x
                    + (2. * maze.cell_size - maze.path_thickness) * direction.y,
                (maze.cell_size - maze.path_thickness) * direction.y
                    + (2. * maze.cell_size - maze.path_thickness) * direction.x,
            );

            commands.spawn((
                Collider::cuboid(shape.x * 0.5, shape.y * 0.5),
                Transform::from_translation(
                    node.position.extend(0.)
                        + Vec3::new(
                            maze.cell_size * direction.x * 0.5,
                            maze.cell_size * direction.y * 0.5,
                            0.,
                        ),
                ),
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: shape * 0.5,
                    },
                },
                Wall,
            ));

            if game_state.get() == &GameState::Paused {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Rectangle {
                            extents: shape,
                            ..default()
                        }),
                        transform: Transform::from_translation(
                            node.position.extend(0.)
                                + Vec3::new(
                                    maze.cell_size * direction.x * 0.5,
                                    maze.cell_size * direction.y * 0.5,
                                    0.,
                                ),
                        ),
                        ..default()
                    },
                    Fill::color(color.wall_color),
                ));
            }
        });
    }
}

fn remove_colliders(
    mut commands: Commands,
    wall_colliders: Query<(Entity, &Transform), With<Wall>>,
    maze: Res<Maze>,
    node_query: Query<&MazeNode>,
) {
    for node in node_query.iter() {
        if node.parent.is_none() {
            continue;
        }
        let parent_node = node_query.get(node.parent.unwrap()).unwrap();
        let parent_direction = parent_node.index - node.index;

        let node_translation = node.position.extend(0.);

        let offset = Vec2::new(
            parent_direction.x * maze.cell_size * 0.5,
            parent_direction.y * maze.cell_size * 0.5,
        );

        for (entity, transform) in wall_colliders.iter() {
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
