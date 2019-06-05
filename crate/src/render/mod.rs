mod colors;
mod lerp;
mod phase_specific_renderers;
mod switch;

use crate::{
    click::Action,
    paint::{Component, ImageType},
    phase::Phase,
    shapes::{rect_button, rect_focus, Translate},
};
use lerp::{LerpInto, Lerpable, Lerper};
use phase_specific_renderers::{BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer};
use switch::{Switch, Switch4};

use nzscq::choices::{Booster, Character};

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
                ..
            } => CharacterChoosingPhaseRenderer {
                completion_factor,
                characters: &currently_available,
            }
            .render(),

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
