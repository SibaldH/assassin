use bevy::prelude::*;

use crate::maze::{Direction, Maze, MazeNode};

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, update_walls); // setup_walls depends on the maze resource
    }
}

#[derive(Component)]
struct Wall {
    direction: Direction,
    position: Vec2,
}

fn update_walls(
    mut commands: Commands,
    maze: ResMut<Maze>,
    node_query: Query<&MazeNode>,
    sprite_query: Query<Entity, With<Wall>>,
    wall_query: Query<&Wall>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Despawn all existing walls
    for wall_entity in sprite_query.iter() {
        commands.entity(wall_entity).despawn();
    }

    for y in 0..maze.grid.len() {
        for x in 0..maze.grid[0].len() {
            draw_walls(
                &mut commands,
                &mut meshes,
                &mut materials,
                Vec2::new(x as f32, y as f32),
                &maze,
            );
        }
    }

    // Despawn all walls in the direction of the parent node
    for node in node_query.iter() {
        if let Some(parent) = node.parent {
            let parent_node = node_query.get(parent).unwrap();
            let parent_direction = parent_node.position - node.position;

            let parent_direction = match (parent_direction.x, parent_direction.y) {
                (0., 1.) => Direction::Up,
                (0., -1.) => Direction::Down,
                (1., 0.) => Direction::Right,
                (-1., 0.) => Direction::Left,
                _ => continue,
            };

            let reverse_direction = match parent_direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };

            //visualize sprite_query
            println!("Sprite query: {:#?}", sprite_query);

            for wall_entity in sprite_query.iter() {
                if let Ok(wall) = wall_query.get(wall_entity) {
                    println!(
                        "Wall position: {:#?} | Direction: {:#?}",
                        wall.position, wall.direction
                    );
                    if wall.direction == parent_direction && wall.position == node.position {
                        commands.entity(wall_entity).despawn();
                    }
                    if wall.direction == reverse_direction && wall.position == parent_node.position
                    {
                        commands.entity(wall_entity).despawn();
                    }
                }
            }
        }
    }
}

fn draw_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    maze: &ResMut<Maze>,
) {
    let wall_thickness = 2.;
    let color = Color::srgb(1.0, 1.0, 1.0);
    let width = maze.grid[0].len() as f32;
    let height = maze.grid.len() as f32;
    let cell_size = maze.cell_size;

    let wall_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    for direction in wall_directions {
        match direction {
            Direction::Up => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(cell_size, wall_thickness))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(
                        position.x * cell_size - width as f32 * 0.5 * cell_size + 0.5 * cell_size,
                        position.y * cell_size - height as f32 * 0.5 * cell_size + cell_size,
                        0.0,
                    )),
                    Wall {
                        direction,
                        position,
                    },
                ));
            }
            Direction::Down => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(cell_size, wall_thickness))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(
                        position.x * cell_size - width * 0.5 * cell_size + 0.5 * cell_size,
                        position.y * cell_size - height * 0.5 * cell_size,
                        0.0,
                    )),
                    Wall {
                        direction,
                        position,
                    },
                ));
            }
            Direction::Left => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(wall_thickness, cell_size))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(
                        position.x * cell_size - width * 0.5 * cell_size,
                        position.y * cell_size - height * 0.5 * cell_size + 0.5 * cell_size,
                        0.0,
                    )),
                    Wall {
                        direction,
                        position,
                    },
                ));
            }
            Direction::Right => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(wall_thickness, cell_size))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(
                        position.x * cell_size - width * 0.5 * cell_size + cell_size,
                        position.y * cell_size - height * 0.5 * cell_size + 0.5 * cell_size,
                        0.0,
                    )),
                    Wall {
                        direction,
                        position,
                    },
                ));
            }
        }
    }
}
