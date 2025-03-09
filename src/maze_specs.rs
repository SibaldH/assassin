use bevy::prelude::*;

#[derive(Resource)]
pub struct MazeColor {
    pub path_color: Color,
    pub wall_color: Color,
    pub root_color: Color,
    pub node_color: Color,
    pub player_color: Color,
}

#[derive(Resource)]
pub struct MazeShape(pub Vec2);
