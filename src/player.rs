use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::maze::Maze;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPlayerTimer(Timer::from_seconds(3., TimerMode::Once)));
        app.add_systems(Update, spawn_player);
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
