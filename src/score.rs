use crate::timer;
use crate::worker_agent;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u64);

impl Score {
    pub fn new() -> Self {
        Score(0)
    }

    pub fn get_score(&self) -> &Score {
        self
    }

    pub fn get_score_value(&self) -> u64 {
        self.0
    }
}

/// Increase the score, if the timers are not paused.
pub fn increment_score_if_unpaused(
    mut score: ResMut<Score>,
    pomodoro_timer: Query<&mut timer::PomodoroTimer>,
    worker_agent: Res<worker_agent::WorkerAgent>,
) {
    let total_worker_score: u8 = worker_agent
        .workers
        .iter()
        .map(|worker| worker.score_increase)
        .sum();

    for timer in pomodoro_timer.iter() {
        // Get the current type of timer, and then check if relevant timer is paused.
        // If not, increment the score.
        match timer.get_current_state() {
            timer::TimerState::Break => {
                if !timer.get_break_timer().paused() {
                    increase_score(&mut score, total_worker_score)
                }
            }
            timer::TimerState::Work => {
                if !timer.get_work_timer().paused() {
                    increase_score(&mut score, total_worker_score)
                }
            }
        }
    }
}

/// Increase the score by 1
pub fn increase_score(score: &mut ResMut<Score>, value: u8) {
    score.0 += value as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_initialization() {
        let score = Score::new();
        assert_eq!(score.get_score().0, 0);
        assert_eq!(score.get_score_value(), 0);
    }

    #[test]
    fn test_increase_score() {
        let mut app = App::new();

        app.insert_resource(Score(0));

        app.add_systems(Update, increase_score);

        app.update();

        assert_eq!(app.world.resource::<Score>().0, 1);
    }
}
