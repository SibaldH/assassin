use bevy::prelude::*;

use crate::gamestate::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTimer(Timer::from_seconds(1., TimerMode::Repeating)));
        app.insert_resource(SprintState {
            sprint_timer: Timer::from_seconds(0.0025, TimerMode::Repeating),
            recovery_timer: Timer::from_seconds(3.0, TimerMode::Once),
            percentage: 100.0,
            change_value: 0.1,
        });
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct ScoreValue(f32);

#[derive(Resource)]
struct ScoreTimer(Timer);

#[derive(Component)]
struct SprintValue;

#[derive(Resource)]
pub struct SprintState {
    pub sprint_timer: Timer,
    pub recovery_timer: Timer,
    pub percentage: f32,
    pub change_value: f32,
}

fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprint_state: Res<SprintState>,
) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-9MyE5.ttf");
    commands
        .spawn(Node {
            width: Val::Percent(90.0),
            height: Val::Percent(20.0),
            align_self: AlignSelf::FlexStart,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect {
                top: Val::Vh(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Score: "),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                    ));

                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                        ScoreValue(0.),
                    ));
                });

            parent
                .spawn((
                    Node {
                        width: Val::Px(30.),
                        height: Val::Px(200.),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::FlexStart,
                        border: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(sprint_state.percentage),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                        SprintValue,
                    ));
                });
        });
}

fn update_hud(
    time: Res<Time>,
    mut timer: ResMut<ScoreTimer>,
    game_state: Res<State<GameState>>,
    mut time_query: Query<(&mut Text, &mut ScoreValue)>,
    sprint_state: Res<SprintState>,
    mut sprint_query: Query<&mut Node, With<SprintValue>>,
) {
    if game_state.get() != &GameState::Running {
        return;
    }

    for mut sprint_bar in &mut sprint_query {
        sprint_bar.height = Val::Percent(sprint_state.percentage);
    }

    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    for (mut time_text, mut time_value) in &mut time_query {
        time_value.0 += 1.;
        time_text.0 = time_value.0.to_string();
    }
}
