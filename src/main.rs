use bevy::prelude::*;

mod timer;

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
        .add_systems(Update, (timer::timer_tick, timer::print_timer, play_sound))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the pomodoro timer
    commands.spawn(timer::PomodoroTimer::new());
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
