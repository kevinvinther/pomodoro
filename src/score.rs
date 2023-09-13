use crate::timer;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u64);

impl Score {
    pub fn new(&self) {
        Score(0);
    }

    pub fn get_score(&self) -> &Score {
        self
    }

    pub fn get_score_value(&self) -> u64 {
        self.0
    }
}

pub fn increase_score(
    mut score: ResMut<Score>, 
    pomodoro_timer: Query<&mut timer::PomodoroTimer>,
) {
    for timer in pomodoro_timer.iter() {

        // TODO: Is there an easier and/or more efficient way to do this?
        // IDEA: Maybe have a function in the implementation that returns?
        //       However, this doesn't really fix the constant "match"es.
        //       I feel like this is too many calculations
        match timer.get_current_state() {
            timer::TimerState::Break => {
                if !timer.get_break_timer().paused() {
                    score.0 += 1;
                }
            },
            timer::TimerState::Work => {
                if !timer.get_work_timer().paused() {
                    score.0 += 1;
                }
            }
            err => panic!("An error has occured, could not get state WORK or BREAK, instead, got: {:?}", err),
        }

    }
}