use nzscq::choices::{Booster, Character};

pub enum Phase {
    ChoosingCharacters {
        previously_available: Vec<Character>,
        currently_available: Vec<Character>,
    },
    ChosenCharacters {
        previously_available: Vec<Character>,
        chosen: Vec<Character>,
    },
    ChoosingBoosters {
        previously_available: Vec<Character>,
        currently_available: Vec<Booster>,
    },
    ChosenBoosters {
        previously_available: Vec<Booster>,
        chosen: Vec<Booster>,
    },
    ChoosingDequeue, //TODO ↓
    ChosenDequeue,
    ChoosingAction,
    ChosenAction,
    GameOver,
}

impl Phase {
    pub fn duration_secs(&self) -> f64 {
        match self {
            Phase::ChoosingCharacters { .. } => durations::CHOOSING_CHARACTERS,
            _ => 0.0, // TODO
        }
    }
}

mod durations {
    pub const CHOOSING_CHARACTERS: f64 = 0.5;
}
