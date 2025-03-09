use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::maze::Maze;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, maze: Res<Maze>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Transform::from_xyz(0., 0., 10.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        GravityScale(0.),
        KinematicCharacterController::default(),
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(maze.cell_size * 0.2),
        Player,
    ));
}

fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_controllers: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut controller = player_controllers.single_mut();
    let mut movement = Vec2::new(0., 0.);

    if keys.pressed(KeyCode::KeyW) {
        movement.y += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement.x += 1.;
    }

    controller.translation = Some(movement);
}
