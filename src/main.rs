use bevy::prelude::*;

mod timer;
mod ui;

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
        .add_systems(Startup, (setup, ui::setup_button, ui::setup_timer_text))
        .add_systems(
            Update,
            (
                timer::timer_tick,
                play_sound,
                ui::button_system,
                ui::work_text_update_system,
                ui::break_text_update_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the pomodoro timer
    commands.spawn(timer::PomodoroTimer::new());
    commands.spawn(Camera2dBundle::default());
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
