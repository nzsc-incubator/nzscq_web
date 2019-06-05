use nzscq::{
    choices::{Booster, Character},
    outcomes::CharacterHeadstart,
};

#[derive(Debug, Clone)]
pub enum Phase {
    ChooseCharacter {
        currently_available: Vec<Character>,
    },
    RechooseCharacter {
        previously_available: Vec<Character>,
        character_headstarts: Vec<CharacterHeadstart>,
        currently_available: Vec<Character>,
    },
    ChooseBooster {
        previously_available: Vec<Character>,
        character_headstarts: Vec<CharacterHeadstart>,
        currently_available: Vec<Booster>,
    },
    ChooseDequeue, //TODO ↓
    ChooseAction,
    GameOver,
}

impl Phase {
    pub fn duration_secs(&self) -> f64 {
        match self {
            Phase::ChooseCharacter { .. } => durations::CHOOSING_CHARACTERS,
            Phase::ChooseBooster { .. } => durations::CHOOSING_BOOSTERS,
            _ => panic!("TODO choose duration"),
        }
    }
}

mod durations {
    pub const CHOOSING_CHARACTERS: f64 = 0.5;
    pub const CHOOSING_BOOSTERS: f64 = 2.0;
}
