use nzscq::{
    choices::{Booster, Character, DequeueChoice},
    outcomes::CharacterHeadstart,
    scoreboard::DequeueingPlayer,
};

#[derive(Debug, Clone)]
pub enum Phase {
    ChooseCharacter {
        available_characters: Vec<Character>,
    },
    RechooseCharacter {
        previously_available_characters: Vec<Character>,
        previously_mutually_chosen_character: Character,
        available_characters: Vec<Character>,
    },
    ChooseBooster {
        previously_available_characters: Vec<Character>,
        previous_outcome: Vec<CharacterHeadstart>,
        available_boosters: Vec<Booster>,
    },
    ChooseFirstDequeue {
        previously_available_boosters: Vec<Booster>,
        scoreboard: [DequeueingPlayer; 2],
        available_dequeues: [Vec<DequeueChoice>; 2],
    },
    ChooseAction, //TODO ↓
    GameOver,
}

impl Phase {
    pub fn duration_secs(&self) -> f64 {
        match self {
            Phase::ChooseCharacter { .. } => durations::CHOOSING_CHARACTERS,
            Phase::RechooseCharacter { .. } => durations::RECHOOSING_CHARACTERS,
            Phase::ChooseBooster { .. } => durations::CHOOSING_BOOSTERS,
            Phase::ChooseFirstDequeue { .. } => durations::CHOOSING_FIRST_DEQUEUE,
            _ => panic!("TODO choose duration"),
        }
    }
}

mod durations {
    pub const CHOOSING_CHARACTERS: f64 = 0.5;
    pub const RECHOOSING_CHARACTERS: f64 = 2.0;
    pub const CHOOSING_BOOSTERS: f64 = 2.5;
    pub const CHOOSING_FIRST_DEQUEUE: f64 = 2.5;
}
