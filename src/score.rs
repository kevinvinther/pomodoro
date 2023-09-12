use crate::timer;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u64);

pub fn increase_score(
    mut score: ResMut<Score>, 
    pomodoro_timer: Query<&mut timer::PomodoroTimer>,
) {
    for timer in pomodoro_timer.iter() {
        if !(timer.get_work_timer().paused() && timer.get_break_timer().paused()) {
            score.0 += 1;
        }
    }
}