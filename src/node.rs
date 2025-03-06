use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::maze::{Maze, MazeNode};

pub struct NodePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for NodePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_nodes.run_if(in_state(self.state.clone())));
    }
}

const CIRCLE_RADIUS_RATIO: f32 = 8.0;

#[derive(Component)]
struct NodeCircle;

#[derive(Component)]
struct RootNodeCircle;

fn setup(mut commands: Commands, query: Query<&Transform>, maze: Res<Maze>) {
    let radius = maze.cell_size / CIRCLE_RADIUS_RATIO;
    for transform in query.iter() {
        let shape = shapes::Circle {
            center: Vec2::new(transform.translation.x, transform.translation.y),
            radius,
        };
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::srgb(0.0, 0.8, 0.0)),
            NodeCircle,
        ));
    }
}

fn update_nodes(
    mut commands: Commands,
    maze: Res<Maze>,
    query: Query<Entity, With<RootNodeCircle>>,
    node_query: Query<(&MazeNode, &Transform)>,
) {
    //Despawn root node circle
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let root_entity = maze.root;
    if let Ok((_root_node, root_transform)) = node_query.get(root_entity) {
        let radius = maze.cell_size / CIRCLE_RADIUS_RATIO;
        let shape = shapes::Circle {
            radius,
            center: Vec2::new(root_transform.translation.x, root_transform.translation.y),
        };
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::srgb(1.0, 0.0, 0.0)),
            RootNodeCircle,
        ));
    }
}
