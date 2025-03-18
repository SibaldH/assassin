use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_darkness);
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct LightMaterial {
    #[uniform(0)]
    resolution: Vec2,
    #[uniform(1)]
    player_pos: Vec2,
    #[uniform(2)]
    light_radius: f32,
}

impl Material for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/light.wgsl".into()
    }
}

fn spawn_darkness(
    mut commands: Commands,
    mut materials: ResMut<Assets<LightMaterial>>,
    window: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let window = window.single();
    let resolution = Vec2::new(window.width(), window.height());

    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(resolution.x, resolution.y)).into())),
        MeshMaterial2d(materials.add(LightMaterial {
            resolution,
            player_pos: Vec2::ZERO,
            light_radius: 0.0,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
