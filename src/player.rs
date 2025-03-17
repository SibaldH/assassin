use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{hud::SprintState, maze::Maze, maze_specs::MazeColor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, maze: Res<Maze>, color: Res<MazeColor>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: maze.cell_size * 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        Fill::color(color.player_color),
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
    time: Res<Time>,
    mut sprint_state: ResMut<SprintState>,
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

    // Normalize movement vector if needed
    let is_moving = movement.length() > 0.0;
    if is_moving {
        movement = movement.normalize() * 2.0;
    }

    // Shift for sprint
    let is_sprinting =
        (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)) && is_moving;
    let mut sprint_factor = 1.0;

    if is_sprinting {
        // Reset recovery timer while sprinting
        sprint_state.recovery_timer.reset();

        // Drain sprint bar only if sprinting and percentage > 0
        if sprint_state.percentage > 0.0 {
            sprint_factor = 1.5;
            if sprint_state.sprint_timer.tick(time.delta()).just_finished() {
                sprint_state.percentage -= sprint_state.change_value;
                sprint_state.percentage = sprint_state.percentage.max(0.0); // Clamp to 0%
            }
        }
    } else {
        // Tick recovery timer when not sprinting
        sprint_state.recovery_timer.tick(time.delta());

        // Recover sprint bar if timer is finished and percentage < 100
        if sprint_state.recovery_timer.finished() && sprint_state.percentage < 100.0 {
            sprint_state.percentage += sprint_state.change_value;
            sprint_state.percentage = sprint_state.percentage.min(100.0); // Clamp to 100%

            // Reset recovery timer if fully recovered
            if sprint_state.percentage >= 100.0 {
                sprint_state.recovery_timer.reset();
            }
        }
    }

    // Apply sprint factor to movement
    controller.translation = Some(movement * sprint_factor);
}
