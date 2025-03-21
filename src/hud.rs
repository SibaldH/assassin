use bevy::prelude::*;

use crate::{gamestate::GameState, player::ManaState};

pub struct HudPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for HudPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTimer(Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Startup, setup_hud.run_if(in_state(self.state.clone())));
        app.add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct ScoreValue(f32);

#[derive(Resource)]
struct ScoreTimer(Timer);

#[derive(Component)]
struct ManaValue;

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>, mana_state: Res<ManaState>) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-9MyE5.ttf");
    commands
        .spawn(Node {
            width: Val::Percent(95.0),
            height: Val::Percent(20.0),
            align_self: AlignSelf::FlexStart,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect {
                top: Val::Vh(2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(100.),
                        height: Val::Px(25.),
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
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                    ));

                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                        ScoreValue(0.),
                    ));
                });

            parent
                .spawn((
                    Node {
                        width: Val::Px(200.),
                        height: Val::Px(30.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        border: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(mana_state.percentage),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                        ManaValue,
                    ));
                });
        });
}

fn update_hud(
    time: Res<Time>,
    mut timer: ResMut<ScoreTimer>,
    game_state: Res<State<GameState>>,
    mut time_query: Query<(&mut Text, &mut ScoreValue)>,
    mana_state: Res<ManaState>,
    mut mana_query: Query<&mut Node, With<ManaValue>>,
) {
    for mut mana_bar in &mut mana_query {
        mana_bar.width = Val::Percent(mana_state.percentage);
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
