use bevy::prelude::*;

mod timer;

const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pomodoro".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_event::<timer::BreakDoneEvent>()
        .add_event::<timer::WorkDoneEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                timer::timer_tick,
                timer::print_timer,
                play_sound,
                button_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the pomodoro timer
    commands.spawn(timer::PomodoroTimer::new());
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(175.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

/// Plays relevant sounds on events
fn play_sound(
    mut ev_break_done: EventReader<timer::BreakDoneEvent>,
    mut ev_work_done: EventReader<timer::WorkDoneEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if !ev_break_done.is_empty() {
        ev_break_done.clear();
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/break_done.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }

    if !ev_work_done.is_empty() {
        ev_work_done.clear();
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/work_done.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

/// Adds a clickable button, which, when pressed either pauses or unpauses currently active timer
#[allow(clippy::type_complexity)]
fn button_system(
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
                *color = PRESSED_BUTTON.into();
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
                *color = HOVERED_BUTTON.into();
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
