use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.insert_resource(PreviousState(None));
        app.add_systems(Update, toggle_pause);
    }
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InGame,
    Pauzed,
    Scanning,
    #[default]
    MainMenu,
}

#[derive(Resource, Debug)]
struct PreviousState(Option<GameState>);

fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut prev_state: ResMut<PreviousState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if current_state.get() != &GameState::Pauzed {
            prev_state.0 = Some(current_state.get().clone());
        }

        match current_state.get() {
            GameState::MainMenu => next_state.set(GameState::Pauzed),
            GameState::Pauzed => next_state.set(prev_state.0.clone().unwrap()),
            GameState::Scanning => next_state.set(GameState::Pauzed),
            GameState::InGame => next_state.set(GameState::Pauzed),
        }
    }
}
