use bevy::prelude::*;

mod timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (timer::timer_tick, timer::print_timer))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the pomodoro timer
    commands.spawn(timer::PomodoroTimer::new());
}
