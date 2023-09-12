use bevy::prelude::*;

use crate::score;
use crate::timer;

const NORMAL_BUTTON: Color = Color::rgb(34.0/255.0, 111.0/255.0, 84.0/255.0);

#[derive(Component)]
pub struct WorkText;

#[derive(Component)]
pub struct BreakText;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                top: Val::Px(105.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        height: Val::Px(25.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: NORMAL_BUTTON.into(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 22.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(15.0),
            bottom: Val::Px(0.0),
            ..default()
        }),
        ScoreText,
    ));
}

pub fn setup_timer_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Break timer
    commands.spawn((
        TextBundle::from_section(
            "00:00",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(55.0),
            left: Val::Px(15.0),
            ..default()
        }),
        BreakText,
    ));

    // Work Timer
    commands.spawn((
        TextBundle::from_section(
            "00:00",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        WorkText,
    ));
}

/// Adds a clickable button, which, when pressed either pauses or unpauses currently active timer
#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut pomodoro_timer: Query<&mut timer::PomodoroTimer>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let paused_status = timer::get_paused_status(&pomodoro_timer);
        match *interaction {
            Interaction::Pressed => {
                match paused_status {
                    Ok(true) => {
                        text.sections[0].value = "Unpause".to_string();
                    }
                    Ok(false) => {
                        text.sections[0].value = "Pause".to_string();
                    }
                    _ => panic!("No timer found"),
                }
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                timer::toggle_timer(&mut pomodoro_timer);
            }
            Interaction::Hovered => {
                match paused_status {
                    Ok(true) => {
                        text.sections[0].value = "Unpause".to_string();
                    }
                    Ok(false) => {
                        text.sections[0].value = "Pause".to_string();
                    }
                    _ => panic!("No timer found"),
                }
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::None => {
                match paused_status {
                    Ok(true) => {
                        text.sections[0].value = "Unpause".to_string();
                    }
                    Ok(false) => {
                        text.sections[0].value = "Pause".to_string();
                    }
                    _ => panic!("No timer found"),
                }
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

/// Updates the "break timer" to the current time remaining in that timer
pub fn break_text_update_system(
    timer: Query<&timer::PomodoroTimer>,
    mut break_text: Query<&mut Text, With<BreakText>>,
) {
    for pomodoro_timer in timer.iter() {
        for mut text in break_text.iter_mut() {
            let formatted = format!(
                "{:02}:{:02}",
                (pomodoro_timer.get_break_timer_remaining_secs() / 60.0).floor(),
                (pomodoro_timer.get_break_timer_remaining_secs() % 60.0).floor(),
            );
            text.sections[0].value = formatted;
        }
    }
}

/// Updates the "work text" to the current time remaining in that timer
pub fn work_text_update_system(
    timer: Query<&timer::PomodoroTimer>,
    mut work_text: Query<&mut Text, With<WorkText>>,
) {
    for pomodoro_timer in timer.iter() {
        for mut text in work_text.iter_mut() {
            let formatted = format!(
                "{:02}:{:02}",
                (pomodoro_timer.get_work_timer_remaining_secs() / 60.0).floor(),
                (pomodoro_timer.get_work_timer_remaining_secs() % 60.0).floor(),
            );
            text.sections[0].value = formatted;
        }
    }
}

pub fn score_text_update_system(
    mut score_text: Query<&mut Text, With<ScoreText>>,
    score: Res<score::Score>,
) {
        for mut text in score_text.iter_mut() {
            text.sections[0].value = format!("{}", score.0);
            text.alignment = TextAlignment::Right;
        }
}
