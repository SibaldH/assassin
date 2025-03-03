use std::f32::consts::PI;

use bevy::prelude::*;
use rand::random_range;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpdateTimer(Timer::from_seconds(
            0.0001,
            TimerMode::Repeating,
        )));
        app.add_systems(Startup, (setup_maze, build_maze).chain());
        app.add_systems(Update, (update_maze, update_arrows));
    }
}

#[derive(Component)]
struct MazeNode {
    position: Vec2,
    parent: Option<Entity>,
}

#[derive(Resource, Debug)]
struct Maze {
    root: Entity,
    grid: Vec<Vec<Entity>>,
    cell_size: f32,
}

fn setup_maze(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut maze = Maze {
        root: Entity::PLACEHOLDER,
        grid: vec![vec![Entity::from_raw(0); 10]; 10],
        cell_size: 50.,
    };

    for y in 0..maze.grid.len() {
        for x in 0..maze.grid[0].len() {
            let entity = commands
                .spawn((
                    MazeNode {
                        position: Vec2::new(x as f32, y as f32),
                        parent: None,
                    },
                    Sprite {
                        image: asset_server.load("circle.png"),
                        custom_size: Some(Vec2::splat(maze.cell_size * 0.25)),
                        color: Color::srgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(
                        x as f32 * maze.cell_size - 10. * 0.5 * maze.cell_size
                            + (maze.cell_size * 0.5),
                        y as f32 * maze.cell_size - 10. * 0.5 * maze.cell_size
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

#[derive(Component)]
struct Arrow;

fn update_arrows(
    mut commands: Commands,
    maze: Res<Maze>,
    query: Query<(&MazeNode, &Transform)>,
    asset_server: Res<AssetServer>,
    arrow_query: Query<Entity, With<Arrow>>,
) {
    // Despawn all existing arrows
    for arrow_entity in arrow_query.iter() {
        commands.entity(arrow_entity).despawn();
    }

    let arrow = asset_server.load("arrow.png");
    for (node, transform) in query.iter() {
        if let Some(parent) = node.parent {
            if let Ok((_parent_node, parent_transform)) = query.get(parent) {
                let direction = parent_transform.translation - transform.translation;
                let angle = direction.y.atan2(direction.x) - PI / 2.0;
                let offset = direction.normalize() * maze.cell_size * 0.5;

                commands.spawn((
                    Sprite {
                        image: arrow.clone(),
                        custom_size: Some(Vec2::splat(maze.cell_size * 0.25)),
                        ..default()
                    },
                    Transform {
                        translation: transform.translation + offset,
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    Arrow,
                ));
            }
        }
    }
}

#[derive(Resource)]
struct UpdateTimer(Timer);

fn update_maze(
    mut maze: ResMut<Maze>,
    mut query: Query<(&mut Sprite, &mut MazeNode)>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
) {
    let root_entity = maze.root;

    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    if let Ok((mut root_sprite, mut root_node)) = query.get_mut(root_entity) {
        root_sprite.color = Color::srgb(0.0, 1.0, 0.0);
        let available_dirs =
            get_available_dir(root_node.position, maze.grid.len(), maze.grid[0].len());

        if !available_dirs.is_empty() {
            let random_index = random_range(0..available_dirs.len());

            let new_root = match available_dirs[random_index] {
                Direction::Up => {
                    maze.grid[root_node.position.y as usize - 1][root_node.position.x as usize]
                }
                Direction::Down => {
                    maze.grid[root_node.position.y as usize + 1][root_node.position.x as usize]
                }
                Direction::Left => {
                    maze.grid[root_node.position.y as usize][root_node.position.x as usize - 1]
                }
                Direction::Right => {
                    maze.grid[root_node.position.y as usize][root_node.position.x as usize + 1]
                }
            };
            root_node.parent = Some(new_root);

            {
                if let Ok((mut new_root_sprite, mut new_root_node)) = query.get_mut(new_root) {
                    new_root_node.parent = None;
                    maze.root = new_root;
                    new_root_sprite.color = Color::srgb(1.0, 0.0, 0.0);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_available_dir(position: Vec2, grid_width: usize, grid_height: usize) -> Vec<Direction> {
    let mut available_dirs = Vec::new();

    if position.y > 0.0 {
        available_dirs.push(Direction::Up);
    }
    if position.y < grid_height as f32 - 1.0 {
        available_dirs.push(Direction::Down);
    }
    if position.x > 0.0 {
        available_dirs.push(Direction::Left);
    }
    if position.x < grid_width as f32 - 1.0 {
        available_dirs.push(Direction::Right);
    }

    available_dirs
}
