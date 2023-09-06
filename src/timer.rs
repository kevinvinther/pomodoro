use bevy::prelude::*;
use std::time::Duration;

/// Represents a pomodoro timer with a specific duration
///
/// The timer keeps track of how long the user should focus on a task or take a break.
#[derive(Component)]
pub struct PomodoroTimer {
    work_timer: Timer,
    break_timer: Timer,
    current_state: TimerState,
}

/// The state of the current timer; whether it is a Work or Break timer.
enum TimerState {
    Work,
    Break,
}

/// Starts a timer from a u64 representing the number of seconds
fn start_timer(seconds: u64) -> Timer {
    Timer::new(Duration::from_secs(seconds), TimerMode::Once)
}

/// Implementation of a Pomodoro timer.
impl PomodoroTimer {
    /// Creates a new Pomodoro timer with a default duration of 25 minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use pomodoro::timer::PomodoroTimer;
    ///
    /// let timer = PomodoroTimer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            work_timer: start_timer(60 * 25),
            break_timer: start_timer(60 * 5),
            current_state: TimerState::Work,
        }
    }

    /// Advances the timer by the given duration and updates the current state accordingly.
    fn tick(&mut self, delta: Duration) {
        match self.current_state {
            TimerState::Work => {
                self.work_timer.tick(delta);

                if self.work_timer.finished() {
                    self.work_timer.reset();
                    self.current_state = TimerState::Break;
                }
            }
            TimerState::Break => {
                self.break_timer.tick(delta);

                if self.break_timer.finished() {
                    self.break_timer.reset();
                    self.current_state = TimerState::Work;
                }
            }
        };
    }
}

/// Starts the timer by running the `tick` function, which ticks the timers.
pub fn run_timer(mut q: Query<(Entity, &mut PomodoroTimer)>, time: Res<Time>) {
    for (_entity, mut pomodoro_timer) in q.iter_mut() {
        pomodoro_timer.tick(time.delta());
    }
}

/// Prints both the work and break timers in the format
/// `25:00   05:00`
/// Where 25:00 is work, and 05:00 is break.
pub fn print_timer(mut q: Query<(Entity, &PomodoroTimer)>) {
    for (_entity, pomodoro_timer) in q.iter_mut() {
        println!(
            "{:02}:{:02} \t {:02}:{:02}",
            (pomodoro_timer.work_timer.remaining_secs() / 60.0).floor(),
            (pomodoro_timer.work_timer.remaining_secs() % 60.0).floor(),
            (pomodoro_timer.break_timer.remaining_secs() / 60.0).floor(),
            (pomodoro_timer.break_timer.remaining_secs() % 60.0).floor()
        );
    }
}