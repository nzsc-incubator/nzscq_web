mod arrow;
mod dequeue_background;
mod health_display;
mod lerp;
mod phase_specific_renderers;
mod switch;

use crate::{paint::Component, phase::Phase};
use phase_specific_renderers::{
    BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer, CharacterRechoosingPhaseRenderer,
    DequeueingPhaseRenderer,
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

            Phase::ChooseDequeue {
                previously_available_boosters,
                scoreboard,
                available_dequeues,
            } => DequeueingPhaseRenderer {
                completion_factor,
                previously_available_boosters,
                scoreboard,
                available_dequeues,
            }
            .render(),

            _ => panic!("Phase renderer not implemented"),
        }
    }
}
