use bevy::prelude::*;

use crate::{game::Score, GameState};

pub struct UIPlugin;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct PlayButton;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_score, show_play_button));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("UI"),
            NodeBundle {
                style: Style {
                    // i_size: Size::all(Val::Percent(100.)),
                    display: Display::Flex,
                    width: Val::Px(1200.),
                    height: Val::Px(716.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn((
                ScoreUI,
                TextBundle::from_section(
                    "Score: 0",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(2.),
                    top: Val::Percent(2.),
                    align_content: AlignContent::Center,
                    ..default()
                }),
            ));
            ui.spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|menu| {
                menu.spawn((
                    PlayButton,
                    Menu,
                    ImageBundle {
                        image: UiImage::new(asset_server.load("play.png")),
                        style: Style {
                            left: Val::Percent(0.),
                            top: Val::Percent(0.),

                            ..default()
                        },
                        ..default()
                    },
                ));
                menu.spawn((
                    Menu,
                    TextBundle::from_section(
                        "Press Space Key to Play",
                        TextStyle {
                            font_size: 50.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Percent(2.),
                            ..default()
                        },
                        align_content: AlignContent::Center,
                        ..default()
                    }),
                ));
            });
        });
}

fn update_score(score: Res<Score>, mut ui: Query<&mut Text, With<ScoreUI>>) {
    let mut ui = ui.single_mut();

    ui.sections[0].value = format!("Score: {}", score.0);
}

fn show_play_button(mut ui: Query<&mut Visibility, With<Menu>>, game_state: Res<State<GameState>>) {
    if !game_state.is_changed() {
        return;
    }
    for mut ui in &mut ui {
        let game_state = game_state.get();
        if *game_state == GameState::Game {
            *ui = Visibility::Hidden;
        } else {
            *ui = Visibility::Visible;
        }
    }
}
