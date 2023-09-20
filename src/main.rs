use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

mod score;
mod timer;
mod ui;
mod worker;
mod worker_agent;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pomodoro".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_event::<timer::BreakDoneEvent>()
        .add_event::<timer::WorkDoneEvent>()
        .add_systems(
            Startup,
            (
                setup,
                ui::setup_button,
                ui::setup_timer_text,
                ui::setup_score,
            ),
        )
        .add_systems(
            Update,
            (
                timer::timer_tick,
                play_sound,
                ui::button_system,
                ui::work_text_update_system,
                ui::break_text_update_system,
                score::increment_score_if_unpaused.run_if(on_timer(Duration::from_secs(1))),
                ui::score_text_update_system.run_if(on_timer(Duration::from_secs(1))),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the pomodoro timer
    commands.spawn(timer::PomodoroTimer::new());
    commands.spawn(Camera2dBundle::default()); // Setup a 2D camera, will be used in the future.
    commands.insert_resource(score::Score::new()); // Set the global resource `score` to be 0.
                                                   // In the future this should be loaded from a savefile
    commands.insert_resource(worker_agent::WorkerAgent::new());
    commands.insert_resource(ClearColor(Color::rgb(
        0.5294117647,
        0.76470588235,
        0.56078431372,
    ))); // Add a background color
}

/// Plays relevant sounds on events
fn play_sound(
    mut ev_break_done: EventReader<timer::BreakDoneEvent>,
    mut ev_work_done: EventReader<timer::WorkDoneEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // If there is a break event
    if !ev_break_done.is_empty() {
        // Remove the event
        ev_break_done.clear();
        // Play the sound
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/break_done.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }

    // If there is a work event
    if !ev_work_done.is_empty() {
        // Remove the event
        ev_work_done.clear();
        // Play the sound
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/work_done.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
