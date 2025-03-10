use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{maze::Maze, player::Player};

pub struct FogPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for FogPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibilityMap>();
        app.add_systems(Update, update_visibility_map);
    }
}

const NUM_RAYS: usize = 90;

#[derive(Resource, Default)]
pub struct VisibilityMap {
    visible_points: Vec<Vec2>,
}

#[derive(Resource)]
struct VisibilityTexture(Handle<Image>);

fn setup_visibility_texture(
    mut commands: Commands,
    maze: Res<Maze>,
    mut images: ResMut<Assets<Image>>,
) {
}

fn update_visibility_map(
    mut visibility_map: ResMut<VisibilityMap>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    rapier_context: WriteRapierContext,
    maze: Res<Maze>,
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
