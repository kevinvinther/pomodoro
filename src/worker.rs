#[derive(Debug, Clone)]
pub struct Worker {
    pub name: String,       // Name for the worker
    pub score_increase: u8, // TODO: Should this be another type?
    pub cost: u32,          // Cost of acquisition
}

impl Worker {
    pub fn new(name: &str, score_increase: u8, cost: u32) -> Self {
        Worker {
            name: name.to_string(),
            score_increase,
            cost,
        }
    }
}
