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
        app.add_systems(Startup, setup_path);
        app.add_systems(Update, update_paths.run_if(in_state(self.state.clone())));
    }
}

const RADIUS_RATIO: f32 = 8.0;

#[derive(Component)]
struct Path;

fn setup_path(mut commands: Commands, maze: Res<Maze>, color: Res<MazeColor>) {
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
}

#[allow(clippy::too_many_arguments)]
fn update_paths(
    mut commands: Commands,
    node_query: Query<&MazeNode>,
    sprite_query: Query<Entity, With<Path>>,
    maze: Res<Maze>,
    color: Res<MazeColor>,
    time: Res<Time>,
    mut timer: ResMut<MazeUpdateTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // Despawn all existing paths
    for path_entity in sprite_query.iter() {
        commands.entity(path_entity).despawn();
    }

    let path_thickness = maze.cell_size * 0.8;

    for node in node_query.iter() {
        if node.parent.is_none() {
            continue;
        }
        let parent_node = node_query.get(node.parent.unwrap()).unwrap();
        let parent_direction = parent_node.position - node.position;

        let node_translation = Vec3::new(
            maze.grid[0].len() as f32 * maze.cell_size * -0.5 + maze.cell_size * 0.5,
            maze.grid.len() as f32 * maze.cell_size * -0.5 + maze.cell_size * 0.5,
            0.0,
        );

        let points: Vec<Vec2> = match (parent_direction.x, parent_direction.y) {
            (0., 1.) => {
                vec![
                    Vec2::new(
                        node.position.x * maze.cell_size - path_thickness * 0.5,
                        node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size - path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size + path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        node.position.x * maze.cell_size + path_thickness * 0.5,
                        node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                ]
            }
            (0., -1.) => {
                vec![
                    Vec2::new(
                        node.position.x * maze.cell_size + path_thickness * 0.5,
                        node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size + path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size - path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        node.position.x * maze.cell_size - path_thickness * 0.5,
                        node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                ]
            }
            (1., 0.) => {
                vec![
                    Vec2::new(
                        node.position.x * maze.cell_size - path_thickness * 0.5,
                        node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size + path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size + path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        node.position.x * maze.cell_size - path_thickness * 0.5,
                        node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                ]
            }
            (-1., 0.) => {
                vec![
                    Vec2::new(
                        node.position.x * maze.cell_size + path_thickness * 0.5,
                        node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size - path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size - path_thickness * 0.5,
                    ),
                    Vec2::new(
                        parent_node.position.x * maze.cell_size - path_thickness * 0.5,
                        parent_node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                    Vec2::new(
                        node.position.x * maze.cell_size + path_thickness * 0.5,
                        node.position.y * maze.cell_size + path_thickness * 0.5,
                    ),
                ]
            }
            _ => {
                vec![]
            }
        };
        let shape = shapes::RoundedPolygon {
            points: points.into_iter().collect(),
            closed: false,
            radius: path_thickness / RADIUS_RATIO,
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
