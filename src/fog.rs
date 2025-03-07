use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

pub struct FogPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for FogPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics);
    }
}

fn setup_physics(mut commands: Commands) {}
