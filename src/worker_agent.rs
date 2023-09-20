use crate::worker;

use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct WorkerAgent {
    pub workers: Vec<worker::Worker>,
    pub available_workers: Vec<worker::Worker>, // Workers not yet acquired
}

impl WorkerAgent {
    pub fn new() -> Self {
        WorkerAgent {
            workers: vec![worker::Worker::new("Default Worker", 1, 0)],
            available_workers: vec![worker::Worker::new("Basic Worker", 2, 100)],
        }
    }

    fn hire_worker(&mut self, worker: worker::Worker) {
        self.workers.push(worker.clone());

        // Remove worker from available_workers
        let worker_index = self
            .available_workers
            .iter()
            .position(|w| w.name == worker.name);

        if let Some(worker_index) = worker_index {
            self.available_workers.remove(worker_index);
        }
    }
}
