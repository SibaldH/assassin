use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    hud::SprintState,
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
};

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(RangeNodes(Vec::new()));
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (update_player, update_range_nodes)
                .chain()
                .run_if(in_state(self.state.clone())),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct RangeNodes(pub Vec<Entity>);

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
        PointLight2d {
            intensity: 20.0,
            radius: maze.view_distance,
            falloff: 10.,
            cast_shadows: true,
            color: Color::WHITE,
        },
        Player,
    ));
}

fn update_range_nodes(
    player_pos: Query<&Transform, With<Player>>,
    mut range_nodes: ResMut<RangeNodes>,
    nodes: Query<(Entity, &Transform), With<MazeNode>>,
    maze: Res<Maze>,
) {
    let player_pos = player_pos.single().translation.truncate();
    let mut new_nodes = Vec::new();
    for (node, transform) in nodes.iter() {
        if transform.translation.distance(player_pos.extend(0.)) < 4.0 * maze.cell_size {
            new_nodes.push(node);
        }
    }
    range_nodes.0 = new_nodes;
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
