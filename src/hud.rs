use bevy::prelude::*;

use crate::gamestate::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTimer(Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct ScoreValue(f32);

#[derive(Resource)]
struct ScoreTimer(Timer);

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-9MyE5.ttf");
    commands
        .spawn(Node {
            width: Val::Percent(75.0),
            height: Val::Percent(20.0),
            align_self: AlignSelf::FlexStart,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: "),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
            ));

            parent.spawn((
                Text::new("0"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                ScoreValue(0.),
            ));
        });
}

fn update_hud(
    time: Res<Time>,
    mut timer: ResMut<ScoreTimer>,
    game_state: Res<State<GameState>>,
    mut time_query: Query<(&mut Text, &mut ScoreValue)>,
) {
    if game_state.get() == &GameState::Running {
        timer.0.tick(time.delta());

        if !timer.0.just_finished() {
            return;
        }

        for (mut time_text, mut time_value) in &mut time_query {
            time_value.0 += 1.;
            time_text.0 = time_value.0.to_string();
        }
    }
}
