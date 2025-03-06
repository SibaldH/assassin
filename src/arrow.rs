use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
};

pub struct ArrowPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for ArrowPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_arrows.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
struct Arrow;

fn update_arrows(
    mut commands: Commands,
    maze: Res<Maze>,
    color: Res<MazeColor>,
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
                        color: color.node_color,
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            2.0,
                        ) + offset,
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    Arrow,
                ));
            }
        }
    }
}
