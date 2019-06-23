use nzscq::{
    choices::{Action, Booster, Character, DequeueChoice},
    outcomes::{ActionPointsDestroyed, CharacterHeadstart},
    scoreboard::{ActionlessPlayer, DequeueingPlayer},
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
    ChooseAction {
        previous_scoreboard: [DequeueingPlayer; 2],
        previously_available_dequeues: [Vec<DequeueChoice>; 2],
        previous_outcome: [DequeueChoice; 2],
        scoreboard: [ActionlessPlayer; 2],
        available_actions: [Vec<Action>; 2],
    },
    ChooseSubsequentDequeue {
        previous_scoreboard: [ActionlessPlayer; 2],
        previously_available_actions: [Vec<Action>; 2],
        previous_outcome: [ActionPointsDestroyed; 2],
        scoreboard: [DequeueingPlayer; 2],
        available_dequeues: [Vec<DequeueChoice>; 2],
    },
    GameOver, //TODO ↓
}

impl Phase {
    pub fn duration_secs(&self) -> f64 {
        match self {
            Phase::ChooseCharacter { .. } => durations::CHOOSING_CHARACTERS,
            Phase::RechooseCharacter { .. } => durations::RECHOOSING_CHARACTERS,
            Phase::ChooseBooster { .. } => durations::CHOOSING_BOOSTERS,
            Phase::ChooseFirstDequeue { .. } => durations::CHOOSING_FIRST_DEQUEUE,
            Phase::ChooseAction { .. } => durations::CHOOSING_ACTION,
            Phase::ChooseSubsequentDequeue { .. } => durations::CHOOSING_SUBSEQUENT_DEQUEUE,
            _ => panic!("TODO choose duration"),
        }
    }
}

mod durations {
    pub const CHOOSING_CHARACTERS: f64 = 0.5;
    pub const RECHOOSING_CHARACTERS: f64 = 2.0;
    pub const CHOOSING_BOOSTERS: f64 = 2.5;
    pub const CHOOSING_FIRST_DEQUEUE: f64 = 2.5;
    pub const CHOOSING_ACTION: f64 = 2.0;
    pub const CHOOSING_SUBSEQUENT_DEQUEUE: f64 = 2.0;
}
