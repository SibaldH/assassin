use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

pub struct FogPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for FogPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics);
        app.add_systems(
            Update,
            print_ball_altitude.run_if(in_state(self.state.clone())),
        );
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
