use crate::timer;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u64);

impl Score {
    #[allow(unused)]
    pub fn new() -> Score {
        Score(0)
    }

    #[allow(unused)]
    pub fn get_score(&self) -> &Score {
        self
    }

    #[allow(unused)]
    pub fn get_score_value(&self) -> u64 {
        self.0
    }
}

/// Increase the score, if the timers are not paused.
pub fn increase_score(mut score: ResMut<Score>, pomodoro_timer: Query<&mut timer::PomodoroTimer>) {
    for timer in pomodoro_timer.iter() {
        // TODO: Is there an easier and/or more efficient way to do this?
        // IDEA: Maybe have a function in the implementation that returns?
        //       However, this doesn't really fix the constant "match"es.
        //       I feel like this is too many calculations
        // Get the current type of timer, and then check if relevant timer is paused.
        // If not, increment the score.
        match timer.get_current_state() {
            timer::TimerState::Break => {
                if !timer.get_break_timer().paused() {
                    score.0 += 1;
                }
            }
            timer::TimerState::Work => {
                if !timer.get_work_timer().paused() {
                    score.0 += 1;
                }
            }
        }
    }
}
