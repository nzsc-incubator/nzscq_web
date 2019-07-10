use crate::paint::Component;
use crate::render::{
    phase_renderers::{
        ActionChoosingPhaseRenderer, BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer,
        CharacterRechoosingPhaseRenderer, FirstDequeueingPhaseRenderer, GameOverPhaseRenderer,
        SubsequentDequeueingPhaseRenderer,
    },
    Render,
};

use nzscq::{
    choices::{Action, Booster, Character, DequeueChoice, Move},
    outcomes::{ActionPointsDestroyed, CharacterHeadstart},
    scoreboard::{ActionlessPlayer, DequeueingPlayer, FinishedPlayer},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn is_elapsed_time_past_completion(&self, elapsed_time: f64) -> bool {
        elapsed_time > self.duration()
    }

    fn duration(&self) -> f64 {
        match self {
            Phase::ChooseCharacter(_) => durations::CHOOSING_CHARACTERS,
            Phase::RechooseCharacter(_) => durations::RECHOOSING_CHARACTERS,
            Phase::ChooseBooster(_) => durations::CHOOSING_BOOSTERS,
            Phase::ChooseFirstDequeue(_) => durations::CHOOSING_FIRST_DEQUEUE,
            Phase::ChooseAction(_) => durations::CHOOSING_ACTION,
            Phase::ChooseSubsequentDequeue(_) => durations::CHOOSING_SUBSEQUENT_DEQUEUE,
            Phase::GameOver(_) => durations::GAME_OVER,
        }
    }

    pub fn wait_for_user_to_choose_move_to_inspect(&mut self) -> Result<(), ()> {
        match self {
            Phase::ChooseFirstDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::WaitingForUserToChooseMove;

                Ok(())
            }
            Phase::ChooseAction(phase) => {
                phase.inspector_state = MoveInspectorState::WaitingForUserToChooseMove;

                Ok(())
            }
            Phase::ChooseSubsequentDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::WaitingForUserToChooseMove;

                Ok(())
            }

            _ => Err(()),
        }
    }

    pub fn inspect_move(&mut self, m: Move) -> Result<(), ()> {
        match self {
            Phase::ChooseFirstDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::Inspecting(m);

                Ok(())
            }

            Phase::ChooseAction(phase) => {
                phase.inspector_state = MoveInspectorState::Inspecting(m);

                Ok(())
            }

            Phase::ChooseSubsequentDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::Inspecting(m);

                Ok(())
            }

            _ => Err(()),
        }
    }

    pub fn stop_inspecting_move(&mut self) -> Result<(), ()> {
        match self {
            Phase::ChooseFirstDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::NotInspecting;

                Ok(())
            }
            Phase::ChooseAction(phase) => {
                phase.inspector_state = MoveInspectorState::NotInspecting;

                Ok(())
            }
            Phase::ChooseSubsequentDequeue(phase) => {
                phase.inspector_state = MoveInspectorState::NotInspecting;

                Ok(())
            }

            _ => Err(()),
        }
    }

    fn completion_factor(&self, animation_start_time: f64, current_time: f64) -> f64 {
        let elapsed_time = current_time - animation_start_time;
        let factor = elapsed_time / self.duration();

        factor.min(1.0)
    }
}

impl Render<(f64, f64)> for Phase {
    fn render(&self, (animation_start_time, current_time): (f64, f64)) -> Vec<Component> {
        let completion_factor = self.completion_factor(animation_start_time, current_time);

        match self {
            Phase::ChooseCharacter(phase) => {
                CharacterChoosingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::RechooseCharacter(phase) => {
                CharacterRechoosingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::ChooseBooster(phase) => {
                BoosterChoosingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::ChooseFirstDequeue(phase) => {
                FirstDequeueingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::ChooseAction(phase) => {
                ActionChoosingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::ChooseSubsequentDequeue(phase) => {
                SubsequentDequeueingPhaseRenderer::new(phase).render(completion_factor)
            }

            Phase::GameOver(phase) => GameOverPhaseRenderer::new(phase).render(completion_factor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChooseCharacterPhase {
    pub available_characters: Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RechooseCharacterPhase {
    pub previously_available_characters: Vec<Character>,
    pub previously_mutually_chosen_character: Character,
    pub available_characters: Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChooseBoosterPhase {
    pub previously_available_characters: Vec<Character>,
    pub previous_outcome: Vec<CharacterHeadstart>,
    pub available_boosters: Vec<Booster>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChooseFirstDequeuePhase {
    pub previously_available_boosters: Vec<Booster>,
    pub scoreboard: [DequeueingPlayer; 2],
    pub available_dequeues: [Vec<DequeueChoice>; 2],
    pub inspector_state: MoveInspectorState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChooseActionPhase {
    pub previous_scoreboard: [DequeueingPlayer; 2],
    pub previously_available_dequeues: [Vec<DequeueChoice>; 2],
    pub previous_outcome: [DequeueChoice; 2],
    pub scoreboard: [ActionlessPlayer; 2],
    pub available_actions: [Vec<Action>; 2],
    pub inspector_state: MoveInspectorState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChooseSubsequentDequeuePhase {
    pub previous_scoreboard: [ActionlessPlayer; 2],
    pub previously_available_actions: [Vec<Action>; 2],
    pub previous_outcome: [ActionPointsDestroyed; 2],
    pub scoreboard: [DequeueingPlayer; 2],
    pub available_dequeues: [Vec<DequeueChoice>; 2],
    pub inspector_state: MoveInspectorState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameOverPhase {
    pub previous_scoreboard: [ActionlessPlayer; 2],
    pub previously_available_actions: [Vec<Action>; 2],
    pub previous_outcome: [ActionPointsDestroyed; 2],
    pub scoreboard: [FinishedPlayer; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveInspectorState {
    NotInspecting,
    WaitingForUserToChooseMove,
    Inspecting(Move),
}

impl MoveInspectorState {
    pub fn move_(self) -> Option<Move> {
        if let MoveInspectorState::Inspecting(m) = self {
            Some(m)
        } else {
            None
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
    pub const GAME_OVER: f64 = 2.5;
}
