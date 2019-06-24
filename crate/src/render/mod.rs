mod arrow;
mod health_display;
mod lerp;
mod phase_specific_renderers;
mod pill;
mod switch;

use crate::{paint::Component, phase::Phase};
use phase_specific_renderers::{
    ActionChoosingPhaseRenderer, BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer,
    CharacterRechoosingPhaseRenderer, FirstDequeueingPhaseRenderer,
    SubsequentDequeueingPhaseRenderer,
};

pub trait Render {
    fn render(&self) -> Vec<Component>;
}

impl Render for (f64, &Phase) {
    fn render(&self) -> Vec<Component> {
        let (completion_factor, phase) = self;
        let completion_factor = *completion_factor;

        match phase {
            Phase::ChooseCharacter {
                available_characters,
            } => CharacterChoosingPhaseRenderer {
                completion_factor,
                available_characters,
            }
            .render(),

            Phase::RechooseCharacter {
                previously_available_characters,
                previously_mutually_chosen_character,
                available_characters,
            } => CharacterRechoosingPhaseRenderer {
                completion_factor,
                previously_available_characters,
                previously_mutually_chosen_character: *previously_mutually_chosen_character,
                available_characters,
            }
            .render(),

            Phase::ChooseBooster {
                previously_available_characters,
                previous_outcome,
                available_boosters,
            } => BoosterChoosingPhaseRenderer {
                completion_factor,
                previously_available_characters,
                previous_outcome,
                available_boosters,
            }
            .render(),

            Phase::ChooseFirstDequeue {
                previously_available_boosters,
                scoreboard,
                available_dequeues,
            } => FirstDequeueingPhaseRenderer {
                completion_factor,
                previously_available_boosters,
                scoreboard,
                available_dequeues,
            }
            .render(),

            Phase::ChooseAction {
                previous_scoreboard,
                previously_available_dequeues,
                previous_outcome,
                scoreboard,
                available_actions,
            } => ActionChoosingPhaseRenderer {
                completion_factor,
                previous_scoreboard,
                previously_available_dequeues,
                previous_outcome,
                scoreboard,
                available_actions,
            }
            .render(),

            Phase::ChooseSubsequentDequeue {
                previous_scoreboard,
                previously_available_actions,
                previous_outcome,
                scoreboard,
                available_dequeues,
            } => SubsequentDequeueingPhaseRenderer {
                completion_factor,
                previous_scoreboard,
                previously_available_actions,
                previous_outcome,
                scoreboard,
                available_dequeues,
            }
            .render(),

            _ => panic!("Phase renderer not implemented"),
        }
    }
}
