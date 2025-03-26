use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    maze::{Direction, Maze, MazeNode},
    maze_specs::MazeColor,
};

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(RangeNodes(Vec::new()));
        app.insert_resource(ManaState {
            mana_timer: Timer::from_seconds(0.0025, TimerMode::Repeating),
            recovery_timer: Timer::from_seconds(3.0, TimerMode::Once),
            percentage: 100.0,
            change_value: 0.1,
        });
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                update_player_state,
                update_player,
                glitch_wall,
                update_range_nodes,
            )
                .chain()
                .run_if(in_state(self.state.clone())),
        );
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    sprint_factor: f32,
    is_sprinting: bool,
    against_wall: Vec<Direction>,
}

#[derive(Resource)]
pub struct ManaState {
    pub mana_timer: Timer,
    pub recovery_timer: Timer,
    pub percentage: f32,
    pub change_value: f32,
}

#[derive(Resource)]
pub struct RangeNodes(pub Vec<Entity>);

fn spawn_player(mut commands: Commands, maze: Res<Maze>, color: Res<MazeColor>) {
    let playersize = maze.cell_size * 0.5;

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(playersize * 0.5, playersize * 0.5),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        Fill::color(color.player_color),
        RigidBody::Dynamic,
        Velocity::default(),
        GravityScale(0.),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        Sleeping::disabled(),
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        Collider::cuboid(playersize * 0.5 * 0.5, playersize * 0.5 * 0.5),
        Player {
            speed: 200.0,
            sprint_factor: 1.5,
            is_sprinting: false,
            against_wall: Vec::new(),
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
    mut mana_state: ResMut<ManaState>,
) {
    let (mut velocity, mut player) = player_controllers.single_mut();

    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyA) && !player.against_wall.contains(&Direction::Left) {
        direction.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) && !player.against_wall.contains(&Direction::Right) {
        direction.x += 1.;
    }
    if keys.pressed(KeyCode::KeyW) && !player.against_wall.contains(&Direction::Up) {
        direction.y += 1.;
    }
    if keys.pressed(KeyCode::KeyS) && !player.against_wall.contains(&Direction::Down) {
        direction.y -= 1.;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    // Shift for sprint
    player.is_sprinting = (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
        && direction != Vec2::ZERO
        && mana_state.percentage > 0.0;

    let mut speed = player.speed;

    if player.is_sprinting {
        // Reset recovery timer while sprinting
        mana_state.recovery_timer.reset();

        // Drain sprint bar only if sprinting and percentage > 0
        if mana_state.mana_timer.tick(time.delta()).just_finished() {
            mana_state.percentage -= mana_state.change_value;
            mana_state.percentage = mana_state.percentage.max(0.0); // Clamp to 0%
        }

        // Sprint speed
        speed *= player.sprint_factor;
    } else {
        // Tick recovery timer when not sprinting
        mana_state.recovery_timer.tick(time.delta());

        // Recover sprint bar if timer is finished and percentage < 100
        if mana_state.recovery_timer.finished() {
            mana_state.percentage += mana_state.change_value;
            mana_state.percentage = mana_state.percentage.min(100.0); // Clamp to 100%
        }
    }

    velocity.linvel = direction * speed;
}

fn update_player_state(
    mut player_query: Query<(&mut Player, &Transform, &Collider)>,
    rapier_context: WriteRapierContext,
) {
    for (mut player, transform, collider) in player_query.iter_mut() {
        let position = transform.translation.truncate();
        let half_extents = collider.as_cuboid().unwrap().half_extents();

        let buffer = 0.1;

        let mut directions = Vec::new();

        let ray_length = half_extents.y + buffer;

        // Check downwards wall
        if rapier_context
            .single()
            .cast_ray(
                position,
                Vec2::new(0.0, -1.0),
                ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            directions.push(Direction::Down);
        }

        //Check upwards wall
        if rapier_context
            .single()
            .cast_ray(
                position,
                Vec2::new(0.0, 1.0),
                ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            directions.push(Direction::Up);
        }

        //Check left wall
        if rapier_context
            .single()
            .cast_ray(
                position,
                Vec2::new(-1.0, 0.0),
                ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            directions.push(Direction::Left);
        }

        // Check right wall
        if rapier_context
            .single()
            .cast_ray(
                position,
                Vec2::new(1.0, 0.0),
                ray_length,
                true,
                QueryFilter::<'_>::exclude_dynamic().exclude_sensors(),
            )
            .is_some()
        {
            directions.push(Direction::Right);
        }

        player.against_wall = directions;
    }
}

fn glitch_wall(
    mut player_query: Query<(&Player, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    maze: Res<Maze>,
    mut mana_state: ResMut<ManaState>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    for (player, mut transform) in player_query.iter_mut() {
        if mana_state.percentage >= 10.0 {
            for dir in player.against_wall.iter() {
                match *dir {
                    Direction::Left => {
                        if !keys.pressed(KeyCode::KeyA) {
                            continue;
                        }
                        transform.translation -= Vec3::new(maze.cell_size, 0., 0.);
                        mana_state.percentage -= 10.0;
                        mana_state.recovery_timer.reset();
                    }
                    Direction::Right => {
                        if !keys.pressed(KeyCode::KeyD) {
                            continue;
                        }
                        transform.translation += Vec3::new(maze.cell_size, 0., 0.);
                        mana_state.percentage -= 10.0;
                        mana_state.recovery_timer.reset();
                    }
                    Direction::Up => {
                        if !keys.pressed(KeyCode::KeyW) {
                            continue;
                        }
                        transform.translation += Vec3::new(0., maze.cell_size, 0.);
                        mana_state.percentage -= 10.0;
                        mana_state.recovery_timer.reset();
                    }
                    Direction::Down => {
                        if !keys.pressed(KeyCode::KeyS) {
                            continue;
                        }
                        transform.translation -= Vec3::new(0., maze.cell_size, 0.);
                        mana_state.percentage -= 10.0;
                        mana_state.recovery_timer.reset();
                    }
                }
            }
        }
    }
}
