use crate::opponent::Difficulty;

#[derive(Debug, Clone)]
pub struct Context {
    pub computer_difficulty: Difficulty,
    pub current_time: f64,
}
