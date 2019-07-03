mod arrow;
mod arsenal_item_display;
pub use arsenal_item_display::arsenal_item_display;
mod health_display;
mod home_button;
pub use home_button::home_button;
mod lerp;
mod phase_specific_renderers;
mod pill;
mod switch;

mod home_screen;
pub use home_screen::home_screen;
mod settings_screen;
pub use settings_screen::settings_screen;
mod custom_seed_screen;
pub use custom_seed_screen::custom_seed_screen;

use crate::{paint::Component, phase::Phase};
use phase_specific_renderers::{
    ActionChoosingPhaseRenderer, BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer,
    CharacterRechoosingPhaseRenderer, FirstDequeueingPhaseRenderer, GameOverPhaseRenderer,
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
            Phase::ChooseCharacter(phase) => {
                CharacterChoosingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::RechooseCharacter(phase) => {
                CharacterRechoosingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::ChooseBooster(phase) => {
                BoosterChoosingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::ChooseFirstDequeue(phase) => {
                FirstDequeueingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::ChooseAction(phase) => {
                ActionChoosingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::ChooseSubsequentDequeue(phase) => {
                SubsequentDequeueingPhaseRenderer::new(phase, completion_factor).render()
            }

            Phase::GameOver(phase) => GameOverPhaseRenderer::new(phase, completion_factor).render(),
        }
    }
}
