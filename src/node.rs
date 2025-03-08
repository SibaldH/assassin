use bevy::prelude::*;

use crate::{
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
};

pub struct NodePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for NodePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_nodes.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
struct NodeCircle;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&MazeNode, &Transform)>,
    maze: Res<Maze>,
    color: Res<MazeColor>,
) {
    let circle = asset_server.load("circle.png");
    for (_node, transform) in query.iter() {
        commands.spawn((
            Sprite {
                image: circle.clone(),
                custom_size: Some(Vec2::splat(maze.cell_size * 0.25)),
                color: color.node_color,
                ..default()
            },
            Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0),
            NodeCircle,
        ));
    }
}

fn update_nodes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Sprite, Entity), With<NodeCircle>>,
    maze_query: Query<(&MazeNode, &Transform, Entity)>,
    maze: Res<Maze>,
    color: Res<MazeColor>,
) {
    // Despawn all existing nodes
    for (_, entity) in query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new nodes
    let circle = asset_server.load("circle.png");
    for (_node, transform, entity) in maze_query.iter() {
        let mut sprite = Sprite {
            image: circle.clone(),
            custom_size: Some(Vec2::splat(maze.cell_size * 0.25)),
            color: color.node_color,
            ..default()
        };

        // Turn the root node red
        if entity == maze.root {
            sprite.color = color.root_color;
        }

        commands.spawn((
            sprite,
            Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0),
            NodeCircle,
        ));
    }
}
