mod lerp;
mod phase_specific_renderers;
mod switch;

use crate::{
    paint::{Component, },
    phase::Phase,
};
use phase_specific_renderers::{BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer,CharacterRechoosingPhaseRenderer};


pub trait Render {
    fn render(&self) -> Vec<Component>;
}

impl Render for (f64, &Phase) {
    fn render(&self) -> Vec<Component> {
        let (completion_factor, phase) = self;
        let completion_factor = *completion_factor;

        match phase {
            Phase::ChooseCharacter {
                currently_available,
            } => CharacterChoosingPhaseRenderer {
                completion_factor,
                characters: &currently_available,
            }
            .render(),

            Phase::RechooseCharacter {
                previously_available_characters,
                previously_mutually_chosen_character,
                available_characters
            } => CharacterRechoosingPhaseRenderer {
                completion_factor,
                previously_available_characters,
                previously_mutually_chosen_character: *previously_mutually_chosen_character,
                available_characters,
            }.render(),

            Phase::ChooseBooster {
                previously_available,
                character_headstarts,
                currently_available,
            } => BoosterChoosingPhaseRenderer {
                completion_factor,
                previously_available_characters: previously_available,
                previous_outcomes: character_headstarts,
                available_boosters: currently_available,
            }
            .render(),

            _ => panic!("Phase renderer not implemented"),
        }
    }
}
