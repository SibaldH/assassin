use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    maze::{Maze, MazeNode},
    maze_specs::MazeColor,
    walls::Wall,
};

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(RangeNodes(Vec::new()));
        app.insert_resource(SprintState {
            sprint_timer: Timer::from_seconds(0.0025, TimerMode::Repeating),
            recovery_timer: Timer::from_seconds(3.0, TimerMode::Once),
            percentage: 100.0,
            change_value: 0.1,
        });
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (update_player, update_range_nodes)
                .chain()
                .run_if(in_state(self.state.clone())),
        );
        app.add_systems(Update, update_player_state);
        app.add_systems(Update, check_player_state);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    sprint_factor: f32,
    is_sprinting: bool,
    jump_force: f32,
    max_jumps: u32,
    jumps_left: u32,
    is_grounded: bool,
    against_wall: bool,
    ground_contacts: u32,
    wall_contacts: u32,
}

#[derive(Resource)]
pub struct SprintState {
    pub sprint_timer: Timer,
    pub recovery_timer: Timer,
    pub percentage: f32,
    pub change_value: f32,
}

#[derive(Resource)]
pub struct RangeNodes(pub Vec<Entity>);

fn spawn_player(mut commands: Commands, maze: Res<Maze>, color: Res<MazeColor>) {
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: maze.cell_size * 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        Fill::color(color.player_color),
        RigidBody::Dynamic,
        Velocity::default(),
        GravityScale(1.),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        Sleeping::disabled(),
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        Collider::ball(maze.cell_size * 0.2),
        Player {
            speed: 200.0,
            sprint_factor: 1.5,
            is_sprinting: false,
            jump_force: maze.cell_size * 10.,
            max_jumps: 2,
            jumps_left: 1000,
            is_grounded: false,
            against_wall: false,
            ground_contacts: 0,
            wall_contacts: 0,
        },
        PointLight2d {
            intensity: 20.0,
            radius: maze.view_distance,
            falloff: 10.,
            cast_shadows: true,
            color: Color::WHITE,
        },
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
    mut player_controllers: Query<(&mut Velocity, &mut Player)>,
    time: Res<Time>,
    mut sprint_state: ResMut<SprintState>,
) {
    let (mut velocity, mut player) = player_controllers.single_mut();

    let mut direction = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        direction -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += 1.;
    }

    // Shift for sprint
    player.is_sprinting = (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
        && direction != 0.0
        && sprint_state.percentage > 0.0;

    let mut speed = player.speed;

    if player.is_sprinting {
        // Reset recovery timer while sprinting
        sprint_state.recovery_timer.reset();

        // Drain sprint bar only if sprinting and percentage > 0
        if sprint_state.sprint_timer.tick(time.delta()).just_finished() {
            sprint_state.percentage -= sprint_state.change_value;
            sprint_state.percentage = sprint_state.percentage.max(0.0); // Clamp to 0%
        }

        // Sprint speed
        speed *= player.sprint_factor;
    } else {
        // Tick recovery timer when not sprinting
        sprint_state.recovery_timer.tick(time.delta());

        // Recover sprint bar if timer is finished and percentage < 100
        if sprint_state.recovery_timer.finished() {
            sprint_state.percentage += sprint_state.change_value;
            sprint_state.percentage = sprint_state.percentage.min(100.0); // Clamp to 100%
        }
    }

    velocity.linvel.x = direction * speed;

    if keys.just_pressed(KeyCode::Space) && player.jumps_left > 0 {
        velocity.linvel.y = player.jump_force;
        player.jumps_left -= 1;
    }
}

fn check_player_state(query: Query<&Player>) {
    for player in query.iter() {
        println!(
            "Grounded: {}, Against Wall: {}",
            player.is_grounded, player.against_wall
        );
    }
}

fn update_player_state(
    mut player_query: Query<(&mut Player, &Transform, &Collider)>,
    rapier_context: RapierContextMut,
) {
    for (mut player, transform, collider) in player_query.iter_mut() {
        let position = transform.translation;

        // Check if grounded (raycast downward)
        player.is_grounded = false;
        let ground_ray_length = collider.as_cuboid().unwrap().half_extents().y + 0.1;
        if let Some((_, toi)) = rapier_context.cast_ray(
            position.truncate(),
            Vec2::new(0.0, -1.0),
            ground_ray_length,
            true,
            QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
        ) {
            if toi <= ground_ray_length {
                player.is_grounded = true;
            }
        }

        // Check if against wall (raycast left and right)
        player.against_wall = false;
        let wall_ray_length = collider.as_cuboid().unwrap().half_extents().x + 0.1;

        // Check left wall
        if rapier_context
            .cast_ray(
                position.truncate(),
                Vec2::new(-1.0, 0.0),
                wall_ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            player.against_wall = true;
        }

        // Check right wall
        if rapier_context
            .cast_ray(
                position.truncate(),
                Vec2::new(1.0, 0.0),
                wall_ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            player.against_wall = true;
        }
    }
}
