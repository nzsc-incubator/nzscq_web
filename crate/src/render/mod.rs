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

use crate::{paint::Component, phase::Phase};
use phase_specific_renderers::{
    ActionChoosingPhaseRenderer, BoosterChoosingPhaseRenderer, CharacterChoosingPhaseRenderer,
    CharacterRechoosingPhaseRenderer, FirstDequeueingPhaseRenderer, GameOverPhaseRenderer,
    SubsequentDequeueingPhaseRenderer,
};

pub trait Render<A> {
    fn render(&self, args: A) -> Vec<Component>;
}

impl Render<f64> for Phase {
    fn render(&self, completion_factor: f64) -> Vec<Component> {
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
