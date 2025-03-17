use std::cmp::min;

use bevy::prelude::*;
use rand::random_range;

use crate::{maze_specs::MazeShape, player::Player, MazeUpdateTimer};

pub struct MazePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MazePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (setup_maze, build_maze).chain());
        app.add_systems(Update, update_maze.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct MazeNode {
    pub position: Vec2,
    pub index: Vec2,
    pub parent: Option<Entity>,
}

#[derive(Resource, Debug)]
pub struct Maze {
    pub root: Entity,
    pub grid: Vec<Vec<Entity>>,
    pub cell_size: f32,
    pub path_thickness: f32,
    pub view_distance: f32,
}

fn setup_maze(mut commands: Commands, shape: Res<MazeShape>, window: Query<&Window>) {
    let window = window.single();

    let cell_size = min(
        (window.width() / shape.0.x) as i32,
        (window.height() / shape.0.y) as i32,
    ) as f32;

    let mut maze = Maze {
        root: Entity::PLACEHOLDER,
        grid: vec![vec![Entity::from_raw(0); shape.0.x as usize]; shape.0.y as usize],
        cell_size,
        path_thickness: cell_size * 0.8,
        view_distance: cell_size * 3.0,
    };

    for y in 0..maze.grid.len() {
        for x in 0..maze.grid[0].len() {
            let entity = commands
                .spawn((
                    MazeNode {
                        position: Vec2::new(
                            x as f32 * maze.cell_size
                                - maze.grid[0].len() as f32 * 0.5 * maze.cell_size
                                + (maze.cell_size * 0.5),
                            y as f32 * maze.cell_size
                                - maze.grid.len() as f32 * 0.5 * maze.cell_size
                                + (maze.cell_size * 0.5),
                        ),
                        index: Vec2::new(x as f32, y as f32),
                        parent: None,
                    },
                    Transform::from_translation(Vec3::new(
                        x as f32 * maze.cell_size
                            - maze.grid[0].len() as f32 * 0.5 * maze.cell_size
                            + (maze.cell_size * 0.5),
                        y as f32 * maze.cell_size - maze.grid.len() as f32 * 0.5 * maze.cell_size
                            + (maze.cell_size * 0.5),
                        0.0,
                    )),
                ))
                .id();
            maze.grid[y][x] = entity;
        }
    }

    commands.insert_resource(maze);
}

fn build_maze(mut maze: ResMut<Maze>, mut query: Query<&mut MazeNode>) {
    for y in 0..maze.grid.len() {
        for x in 0..maze.grid[0].len() {
            // check if there is a next node in x direction
            if x + 1 < maze.grid[0].len() {
                let next_node = maze.grid[y][x + 1];
                if let Ok(mut current_node) = query.get_mut(maze.grid[y][x]) {
                    current_node.parent = Some(next_node)
                }
            }
        }
        // check if there is a next node in -y direction
        if y + 1 < maze.grid.len() {
            let next_node = maze.grid[y + 1][maze.grid[0].len() - 1];
            if let Ok(mut current_node) = query.get_mut(maze.grid[y][maze.grid[0].len() - 1]) {
                current_node.parent = Some(next_node)
            }
        }
    }

    // set root
    maze.root = maze.grid[maze.grid.len() - 1][maze.grid[0].len() - 1];
}

fn update_maze(
    mut maze: ResMut<Maze>,
    mut query: Query<&mut MazeNode>,
    time: Res<Time>,
    mut timer: ResMut<MazeUpdateTimer>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if let Ok(mut root_node) = query.get_mut(maze.root) {
        let player_pos = player_query.single().translation.truncate();
        let available_dirs = get_available_dir(
            root_node.index,
            root_node.position,
            (maze.grid[0].len(), maze.grid.len()),
            &maze,
            player_pos,
        );

        if !available_dirs.is_empty() {
            let random_index = random_range(0..available_dirs.len());

            let new_root = match available_dirs[random_index] {
                Direction::Up => {
                    maze.grid[root_node.index.y as usize - 1][root_node.index.x as usize]
                }
                Direction::Down => {
                    maze.grid[root_node.index.y as usize + 1][root_node.index.x as usize]
                }
                Direction::Left => {
                    maze.grid[root_node.index.y as usize][root_node.index.x as usize - 1]
                }
                Direction::Right => {
                    maze.grid[root_node.index.y as usize][root_node.index.x as usize + 1]
                }
            };
            root_node.parent = Some(new_root);

            {
                if let Ok(mut new_root_node) = query.get_mut(new_root) {
                    new_root_node.parent = None;
                    maze.root = new_root;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_available_dir(
    root_index: Vec2,
    root_position: Vec2,
    maze_shape: (usize, usize),
    maze: &Maze,
    player_position: Vec2,
) -> Vec<Direction> {
    let mut available_dirs = Vec::new();

    if root_index.y > 0.0
        && player_position.distance(root_position + Vec2::new(0., maze.cell_size))
            > maze.view_distance
    {
        available_dirs.push(Direction::Up);
    }
    if root_index.y < maze_shape.1 as f32 - 1.0
        && player_position.distance(root_position + Vec2::new(0., -maze.cell_size))
            > maze.view_distance
    {
        available_dirs.push(Direction::Down);
    }
    if root_index.x > 0.0
        && player_position.distance(root_position + Vec2::new(-maze.cell_size, 0.))
            > maze.view_distance
    {
        available_dirs.push(Direction::Left);
    }
    if root_index.x < maze_shape.0 as f32 - 1.0
        && player_position.distance(root_position + Vec2::new(maze.cell_size, 0.))
            > maze.view_distance
    {
        available_dirs.push(Direction::Right);
    }

    available_dirs
}
