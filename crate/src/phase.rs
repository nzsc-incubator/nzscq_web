use nzscq::{
    choices::{Action, Booster, Character, DequeueChoice},
    outcomes::{ActionPointsDestroyed, CharacterHeadstart},
    scoreboard::{ActionlessPlayer, DequeueingPlayer, FinishedPlayer},
};

#[derive(Debug, Clone)]
pub enum Phase {
    ChooseCharacter(ChooseCharacterPhase),
    RechooseCharacter(RechooseCharacterPhase),
    ChooseBooster(ChooseBoosterPhase),
    ChooseFirstDequeue(ChooseFirstDequeuePhase),
    ChooseAction(ChooseActionPhase),
    ChooseSubsequentDequeue(ChooseSubsequentDequeuePhase),
    GameOver(GameOverPhase),
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
            Phase::GameOver { .. } => durations::GAME_OVER,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChooseCharacterPhase {
    pub available_characters: Vec<Character>,
}

#[derive(Debug, Clone)]
pub struct RechooseCharacterPhase {
    pub previously_available_characters: Vec<Character>,
    pub previously_mutually_chosen_character: Character,
    pub available_characters: Vec<Character>,
}

#[derive(Debug, Clone)]
pub struct ChooseBoosterPhase {
    pub previously_available_characters: Vec<Character>,
    pub previous_outcome: Vec<CharacterHeadstart>,
    pub available_boosters: Vec<Booster>,
}

#[derive(Debug, Clone)]
pub struct ChooseFirstDequeuePhase {
    pub previously_available_boosters: Vec<Booster>,
    pub scoreboard: [DequeueingPlayer; 2],
    pub available_dequeues: [Vec<DequeueChoice>; 2],
}

#[derive(Debug, Clone)]
pub struct ChooseActionPhase {
    pub previous_scoreboard: [DequeueingPlayer; 2],
    pub previously_available_dequeues: [Vec<DequeueChoice>; 2],
    pub previous_outcome: [DequeueChoice; 2],
    pub scoreboard: [ActionlessPlayer; 2],
    pub available_actions: [Vec<Action>; 2],
}

#[derive(Debug, Clone)]
pub struct ChooseSubsequentDequeuePhase {
    pub previous_scoreboard: [ActionlessPlayer; 2],
    pub previously_available_actions: [Vec<Action>; 2],
    pub previous_outcome: [ActionPointsDestroyed; 2],
    pub scoreboard: [DequeueingPlayer; 2],
    pub available_dequeues: [Vec<DequeueChoice>; 2],
}

#[derive(Debug, Clone)]
pub struct GameOverPhase {
    pub previous_scoreboard: [ActionlessPlayer; 2],
    pub previously_available_actions: [Vec<Action>; 2],
    pub previous_outcome: [ActionPointsDestroyed; 2],
    pub scoreboard: [FinishedPlayer; 2],
}

mod durations {
    pub const CHOOSING_CHARACTERS: f64 = 0.5;
    pub const RECHOOSING_CHARACTERS: f64 = 2.0;
    pub const CHOOSING_BOOSTERS: f64 = 2.5;
    pub const CHOOSING_FIRST_DEQUEUE: f64 = 2.5;
    pub const CHOOSING_ACTION: f64 = 2.0;
    pub const CHOOSING_SUBSEQUENT_DEQUEUE: f64 = 2.0;
    pub const GAME_OVER: f64 = 2.5;
}
