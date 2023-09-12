use bevy::prelude::*;
use std::time::Duration;

/// Event for when the break session has finished
#[derive(Event, Resource)]
pub struct BreakDoneEvent;

/// Event for when the work session has finished
#[derive(Event)]
pub struct WorkDoneEvent;

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

    /// Gets the work timer.
    ///
    /// # Returns
    ///
    /// * `Timer`: The work timer.
    pub fn get_work_timer(&self) -> &Timer {
        &self.work_timer
    }

    /// Gets the break timer.
    ///
    /// # Returns
    ///
    /// * `Timer`: The break timer.
    pub fn get_break_timer(&self) -> &Timer {
        &self.break_timer
    }

    /// Gets the remaining seconds for the work timer
    ///
    /// # Returns
    /// * `f32`: The remaining seconds for the timer
    pub fn get_work_timer_remaining_secs(&self) -> f32 {
        self.work_timer.remaining_secs()
    }

    /// Gets the remaining seconds for the break timer
    ///
    /// # Returns
    /// * `f32`: The remaining seconds for the timer
    pub fn get_break_timer_remaining_secs(&self) -> f32 {
        self.break_timer.remaining_secs()
    }
}

/// Advances the timer by the given duration and updates the current state accordingly.
pub fn timer_tick(
    mut pomodoro_timer: Query<&mut PomodoroTimer>,
    time: Res<Time>,
    mut work_done_event: EventWriter<WorkDoneEvent>,
    mut break_done_event: EventWriter<BreakDoneEvent>,
) {
    for mut entity in pomodoro_timer.iter_mut() {
        match entity.current_state {
            TimerState::Work => {
                entity.work_timer.tick(time.delta());

                if entity.work_timer.finished() {
                    entity.work_timer.reset();
                    entity.current_state = TimerState::Break;
                    work_done_event.send(WorkDoneEvent);
                }
            }
            TimerState::Break => {
                entity.break_timer.tick(time.delta());

                if entity.break_timer.finished() {
                    entity.break_timer.reset();
                    entity.current_state = TimerState::Work;
                    break_done_event.send(BreakDoneEvent);
                }
            }
        };
    }
}

/// Pause or unpause the timer depending on its current state
pub fn toggle_timer(q: &mut Query<&mut PomodoroTimer>) {
    for mut pomodoro_timer in q.iter_mut() {
        match pomodoro_timer.current_state {
            TimerState::Work => {
                if pomodoro_timer.work_timer.paused() {
                    pomodoro_timer.work_timer.unpause();
                } else {
                    pomodoro_timer.work_timer.pause();
                }
            }
            TimerState::Break => {
                if pomodoro_timer.break_timer.paused() {
                    pomodoro_timer.break_timer.unpause();
                } else {
                    pomodoro_timer.break_timer.pause();
                }
            }
        }
    }
}

/// Get the status of whether or not the timer is paused
pub fn get_paused_status(q: &Query<&mut PomodoroTimer>) -> Result<bool, &'static str> {
    if let Some(pomodoro_timer) = q.iter().next() {
        match pomodoro_timer.current_state {
            TimerState::Break => {
                return Ok(pomodoro_timer.break_timer.paused());
            }
            TimerState::Work => {
                return Ok(pomodoro_timer.work_timer.paused());
            }
        }
    }
    Err("No timer found")
}
