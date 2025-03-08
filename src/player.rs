use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::maze::Maze;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPlayerTimer(Timer::from_seconds(3., TimerMode::Once)));
        app.add_systems(Update, (spawn_player, log_entities));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
struct SpawnPlayerTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    maze: Res<Maze>,
    positions: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnPlayerTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    commands.spawn((
        RigidBody::Dynamic,
        Transform::from_xyz(0., 0., 10.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        GravityScale(0.),
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(maze.cell_size * 0.2),
        ColliderMassProperties::Density(2.0),
        Player,
    ));
    for position in positions.iter() {
        println!(
            "Player position:\n\tx: {}\n\ty: {}",
            position.translation.x, position.translation.y
        );
    }
}

fn log_entities(
    window: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    entities: Query<(Entity, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    maze: Res<Maze>,
) {
    let window = window.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if mouse.pressed(MouseButton::Left) {
            if let Ok((camera, camera_transform)) = cameras.get_single() {
                let world_pos = camera
                    .viewport_to_world_2d(camera_transform, cursor_pos)
                    .unwrap_or(Vec2::ZERO);

                info!("Cursor pos: {:#?}", world_pos);

                for (entity, transform) in entities.iter() {
                    let entity_pos = transform.translation().truncate();

                    let hit_radius = maze.cell_size * 0.4;
                    if entity_pos.distance(world_pos) < hit_radius {
                        info!("Hit entity: {:#?}", entity);
                    }
                }
            }
        };
    }
}
