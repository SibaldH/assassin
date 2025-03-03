use bevy::prelude::*;

use crate::MAZE;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, wall_setup);
    }
}

#[derive(Component)]
pub struct Wall;

fn wall_setup(
    mut commands: Commands,
    mut meches: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall_material = materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 0.5)));
    let wall_size = Vec2::new(40., 40.);

    for (y, row) in MAZE.trim().split('\n').enumerate() {
        for (x, cell) in row.split_whitespace().enumerate() {
            if cell == "1" {
                commands.spawn((
                    Sprite {
                        custom_size: Some(wall_size),
                        ..default()
                    },
                    Transform::from_xyz(x as f32 * wall_size.x, y as f32 * wall_size.y, 0.),
                    Wall,
                ));
            }
        }
    }
}
