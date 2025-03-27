use std::collections::HashMap;

use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::maze::{Direction, Maze, MazeNode};

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(FirstRunTracker(false));
        app.insert_resource(RangeNodes(Vec::new()));
        app.insert_resource(ManaState {
            mana_timer: Timer::from_seconds(0.0025, TimerMode::Repeating),
            recovery_timer: Timer::from_seconds(3.0, TimerMode::Once),
            percentage: 100.0,
            change_value: 0.1,
        });
        app.add_systems(OnEnter(self.state.clone()), spawn_player);
        app.add_systems(
            Update,
            (
                update_player_state,
                update_player,
                update_player_animation,
                animate_player_sprite,
                glitch_wall,
                update_range_nodes,
            )
                .chain()
                .run_if(in_state(self.state.clone())),
        );
    }
}

const PLAYER_FPS: u8 = 8;

#[derive(Component)]
pub struct Player {
    speed: f32,
    sprint_factor: f32,
    is_sprinting: bool,
    against_wall: Vec<Direction>,
    state: PlayerState,
    direction: Direction,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum PlayerState {
    Idle,
    Walking,
    Hurt,
    Death,
}

#[derive(Component, Clone, Debug)]
struct PlayerAnimation {
    first_index: usize,
    last_index: usize,
    frame_timer: Timer,
    flip_x: bool,
}

impl PlayerAnimation {
    fn new(first_index: usize, last_index: usize, flip_x: bool) -> Self {
        Self {
            first_index,
            last_index,
            frame_timer: Self::timer_from_fps(PLAYER_FPS),
            flip_x,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::from_seconds(1. / fps as f32, TimerMode::Once)
    }
}

#[derive(Component)]
struct PlayerAnimations {
    animations: HashMap<(PlayerState, Direction), PlayerAnimation>,
    current_animation: PlayerAnimation,
}

impl PlayerAnimations {
    fn new() -> Self {
        let mut animations = HashMap::new();

        animations.insert(
            (PlayerState::Idle, Direction::Down),
            PlayerAnimation::new(0, 1, false),
        );
        animations.insert(
            (PlayerState::Idle, Direction::Left),
            PlayerAnimation::new(4, 5, true),
        );
        animations.insert(
            (PlayerState::Idle, Direction::Right),
            PlayerAnimation::new(4, 5, false),
        );
        animations.insert(
            (PlayerState::Idle, Direction::Up),
            PlayerAnimation::new(8, 9, false),
        );

        let default_animation = animations[&(PlayerState::Idle, Direction::Up)].clone();

        Self {
            animations,
            current_animation: default_animation,
        }
    }

    fn update_animation(&mut self, state: PlayerState, dir: Direction) {
        if let Some(animation) = self.animations.get(&(state, dir)) {
            self.current_animation = animation.clone()
        }
    }
}

#[derive(Resource)]
struct FirstRunTracker(bool);

#[derive(Resource)]
pub struct ManaState {
    pub mana_timer: Timer,
    pub recovery_timer: Timer,
    pub percentage: f32,
    pub change_value: f32,
}

#[derive(Resource)]
pub struct RangeNodes(pub Vec<Entity>);

fn update_player_animation(mut query: Query<(&Player, &mut PlayerAnimations)>) {
    for (player, mut animations) in query.iter_mut() {
        animations.update_animation(player.state.clone(), player.direction);
    }
}

fn animate_player_sprite(time: Res<Time>, mut query: Query<(&mut Sprite, &mut PlayerAnimations)>) {
    for (mut sprite, mut animations) in query.iter_mut() {
        let animation = &mut animations.current_animation;
        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= animation.last_index {
                    animation.first_index
                } else {
                    atlas.index + 1
                }
            }

            animation.frame_timer.reset();
        }
        sprite.flip_x = animation.flip_x;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut run_once: ResMut<FirstRunTracker>,
    maze: Res<Maze>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !run_once.0 {
        run_once.0 = true;
    } else {
        return;
    }

    let image_handle: Handle<Image> =
        asset_server.load("sprite/character/Prototype_Character_Blue.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        4,
        12,
        Some(UVec2::splat(16)),
        Some(UVec2::splat(8)),
    );
    let texture_atlas_layout = texture_atlases.add(layout);

    let player_animations = PlayerAnimations::new();

    commands.spawn((
        PointLight2d {
            intensity: 20.0,
            radius: maze.view_distance,
            falloff: 10.,
            cast_shadows: true,
            color: Color::WHITE,
        },
        Sprite {
            image: image_handle,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animations.current_animation.first_index,
            }),
            flip_x: player_animations.current_animation.flip_x,
            ..default()
        },
        player_animations,
        RigidBody::Dynamic,
        Velocity::default(),
        GravityScale(0.),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        Sleeping::disabled(),
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        Collider::cuboid(16. * 0.5, 16. * 0.5),
        Player {
            speed: 200.0,
            sprint_factor: 1.5,
            is_sprinting: false,
            against_wall: Vec::new(),
            state: PlayerState::Idle,
            direction: Direction::Down,
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

    // Set player direction | Prioritize left and right over up and down when moving diagonally
    if direction.x > 0. {
        player.direction = Direction::Right;
    } else if direction.x < 0. {
        player.direction = Direction::Left;
    } else if direction.y > 0. {
        player.direction = Direction::Up;
    } else if direction.y < 0. {
        player.direction = Direction::Down;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        player.state = PlayerState::Walking;
    } else {
        player.state = PlayerState::Idle;
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
