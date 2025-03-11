use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::maze::Maze;
use crate::maze_specs::MazeColor;
use crate::player::Player;
use crate::walls::Wall;

pub struct FogPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for FogPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibilityMap>();
        app.add_systems(Update, (update_visibility, update_node_visibility).chain());
    }
}

const NUM_RAYS: usize = 90;

#[derive(Resource, Default)]
pub struct VisibilityMap {
    visible_points: Vec<Vec2>,
}

fn update_visibility(
    mut visibility_map: ResMut<VisibilityMap>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    rapier_context: WriteRapierContext,
    maze: Res<Maze>,
    _gizmos: Gizmos,
) {
    let (player_entity, player_transform) = player_query.single();
    let player_pos = player_transform.translation.truncate(); // 2D position

    // Clear previous visibility
    visibility_map.visible_points.clear();

    for i in 0..NUM_RAYS {
        let angle = (i as f32) * (2.0 * std::f32::consts::PI / NUM_RAYS as f32);
        let direction = Vec2::new(angle.cos(), angle.sin());

        // Cast a ray using bevy_rapier
        if let Some((_, toi)) = rapier_context.single().cast_ray(
            player_pos,
            direction,
            maze.view_distance,
            false,
            QueryFilter::new().exclude_collider(player_entity),
        ) {
            let hit_point = player_pos + direction * toi;
            visibility_map.visible_points.push(hit_point);
        } else {
            let visible_point = player_pos + direction * maze.view_distance;
            visibility_map.visible_points.push(visible_point);
        }
    }
}

fn update_node_visibility(
    mut commands: Commands,
    visibility_map: Res<VisibilityMap>,
    maze: Res<Maze>,
    mut wall_query: Query<(&Transform, Entity, &mut Wall)>,
    color: Res<MazeColor>,
) {
    // Update node visibility
    for (wall_transform, wall_entity, wall) in wall_query.iter_mut() {
        let tile_pos = Vec2::new(wall_transform.translation.x, wall_transform.translation.y);

        if visibility_map
            .visible_points
            .iter()
            .any(|&point| (point - tile_pos).length() < maze.cell_size * 0.5)
        {
            let shape = shapes::Rectangle {
                extents: Vec2::new(
                    (maze.cell_size - maze.path_thickness) * wall.direction.x
                        + maze.path_thickness * wall.direction.y,
                    (maze.cell_size - maze.path_thickness) * wall.direction.y
                        + maze.path_thickness * wall.direction.x,
                ),
                ..default()
            };
            commands.entity(wall_entity).try_insert((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_translation(Vec3::new(
                        wall_transform.translation.x,
                        wall_transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
                Fill::color(color.wall_color),
            ));
        }
    }
}
