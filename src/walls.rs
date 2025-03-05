use std::f32::consts::PI;

use bevy::prelude::*;

use crate::maze::{Maze, MazeNode, UpdateTimer};

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_path, update_paths));
        app.add_systems(Update, update_paths);
    }
}

const RADIUS_RATIO: f32 = 8.0;
const WALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const PATH_COLOR: Color = Color::srgb(0.2, 0.2, 0.2); //grey
const PATH_THICKNESS: f32 = 5.;

#[derive(Component)]
struct Path;

fn setup_path(
    mut commands: Commands,
    maze: Res<Maze>,
    node_query: Query<&MazeNode>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Draw a filled in box around the maze
    let radius = maze.cell_size / RADIUS_RATIO;

    // Background
    // Top
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(
            radius,
            maze.grid[0].len() as f32 * maze.cell_size,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform {
            translation: Vec3::new(0., maze.grid.len() as f32 * maze.cell_size / 2., -1.),
            rotation: Quat::from_rotation_z(PI / 2.0),
            ..default()
        },
    ));

    // Bottom
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(
            radius,
            maze.grid[0].len() as f32 * maze.cell_size,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform {
            translation: Vec3::new(0., maze.grid.len() as f32 * maze.cell_size / -2., -1.),
            rotation: Quat::from_rotation_z(PI / 2.0),
            ..default()
        },
    ));

    // Left
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(
            radius,
            maze.grid.len() as f32 * maze.cell_size,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform {
            translation: Vec3::new(maze.grid[0].len() as f32 * maze.cell_size / -2., 0., -1.),
            ..default()
        },
    ));

    // Right
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(
            radius,
            maze.grid.len() as f32 * maze.cell_size,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform {
            translation: Vec3::new(maze.grid[0].len() as f32 * maze.cell_size / 2., 0., -1.),
            ..default()
        },
    ));

    // Infill
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            maze.grid[0].len() as f32 * maze.cell_size - radius / 2.0,
            maze.grid.len() as f32 * maze.cell_size - radius / 2.0,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform::from_translation(Vec3::new(0., 0., -1.)),
    ));

    for node in node_query.iter() {
        let node_translation = Vec3::new(
            node.position.x * maze.cell_size - maze.grid[0].len() as f32 * maze.cell_size / 2.0
                + (maze.cell_size * 0.5),
            node.position.y * maze.cell_size - maze.grid.len() as f32 * maze.cell_size / 2.0
                + (maze.cell_size * 0.5),
            0.0,
        );

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                maze.cell_size - PATH_THICKNESS * 0.5,
                maze.cell_size - PATH_THICKNESS * 0.5,
            ))),
            MeshMaterial2d(materials.add(PATH_COLOR)),
            Transform::from_translation(node_translation),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn update_paths(
    mut commands: Commands,
    node_query: Query<&MazeNode>,
    sprite_query: Query<Entity, With<Path>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    maze: Res<Maze>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // Despawn all existing paths
    for path_entity in sprite_query.iter() {
        commands.entity(path_entity).despawn();
    }

    for node in node_query.iter() {
        let node_translation = Vec3::new(
            node.position.x * maze.cell_size - maze.grid[0].len() as f32 * maze.cell_size / 2.0
                + (maze.cell_size * 0.5),
            node.position.y * maze.cell_size - maze.grid.len() as f32 * maze.cell_size / 2.0
                + (maze.cell_size * 0.5),
            0.0,
        );

        if let Some(parent) = node.parent {
            let parent_node = node_query.get(parent).unwrap();
            let parent_direction = parent_node.position - node.position;

            match (parent_direction.x, parent_direction.y) {
                (0., 1.) => {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            maze.cell_size - PATH_THICKNESS * 0.5,
                            PATH_THICKNESS,
                        ))),
                        MeshMaterial2d(materials.add(PATH_COLOR)),
                        Transform::from_translation(
                            node_translation + Vec3::new(0., (maze.cell_size) * 0.5, 0.),
                        ),
                        Path,
                    ));
                }
                (0., -1.) => {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            maze.cell_size - PATH_THICKNESS * 0.5,
                            PATH_THICKNESS,
                        ))),
                        MeshMaterial2d(materials.add(PATH_COLOR)),
                        Transform::from_translation(
                            node_translation + Vec3::new(0., (maze.cell_size) * -0.5, 0.0),
                        ),
                        Path,
                    ));
                }
                (1., 0.) => {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            PATH_THICKNESS,
                            maze.cell_size - PATH_THICKNESS * 0.5,
                        ))),
                        MeshMaterial2d(materials.add(PATH_COLOR)),
                        Transform::from_translation(
                            node_translation + Vec3::new((maze.cell_size) * 0.5, 0., 0.),
                        ),
                        Path,
                    ));
                }
                (-1., 0.) => {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            PATH_THICKNESS,
                            maze.cell_size - PATH_THICKNESS * 0.5,
                        ))),
                        MeshMaterial2d(materials.add(PATH_COLOR)),
                        Transform::from_translation(
                            node_translation + Vec3::new(maze.cell_size * -0.5, 0., 0.),
                        ),
                        Path,
                    ));
                }
                _ => continue,
            };
        }
    }
}
