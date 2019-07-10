mod arrow;
mod arsenal_item_display;
pub use arsenal_item_display::arsenal_item_display;
mod health_display;
mod home_button;
pub use home_button::home_button;
mod lerp;
pub mod phase_renderers;
mod pill;
mod switch;

mod home_screen;
pub use home_screen::home_screen;
mod settings_screen;
pub use settings_screen::settings_screen;
mod move_inspector;
pub mod move_inspector_buttons;

use crate::paint::Component;

pub trait Render<A> {
    fn render(&self, args: A) -> Vec<Component>;
}
