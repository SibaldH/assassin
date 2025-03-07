use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, toggle_pause);
    }
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match current_state.get() {
            GameState::Paused => next_state.set(GameState::Running),
            GameState::Running => next_state.set(GameState::Paused),
        }
    }
}
