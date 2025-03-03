use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimention: Vec2,
    pub pixel_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    commands.insert_resource(Resolution {
        screen_dimention: Vec2::new(window.width(), window.height()),
        pixel_ratio: 15.,
    });
}
