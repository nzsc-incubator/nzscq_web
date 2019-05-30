mod colors;
mod lerp;

use crate::{
    paint::{Component, ImageType},
    phase::Phase,
    shapes::{rect_button, Translate},
};
use lerp::{LerpInto, Lerpable, Lerper};

pub trait Render {
    fn render(&self) -> Vec<Component>;
}

impl Render for (f64, &Phase) {
    fn render(&self) -> Vec<Component> {
        let (completion_factor, phase) = self;

        match phase {
            Phase::ChoosingCharacters {
                currently_available,
                ..
            } => {
                let lerper = Lerper::from_completion_factor(*completion_factor);
                let mut components = vec![Component::Background(colors::BACKGROUND)];
                let character_buttons: Vec<Component> = currently_available
                    .iter()
                    .enumerate()
                    .map(|(i, character)| {
                        vec![
                            Lerpable::Rect {
                                fill_color: colors::character_color(character),
                                start: rect_button::background_at(i).translate(900.0, 0.0),
                                end: rect_button::background_at(i),
                            },
                            Lerpable::Image {
                                image_type: ImageType::Character(*character),
                                start: rect_button::foreground_at(i).translate(900.0, 0.0),
                                end: rect_button::foreground_at(i),
                            },
                        ]
                        .into_iter()
                    })
                    .flatten()
                    .map(|lerpable| lerpable.lerp(&lerper))
                    .collect();
                components.extend(character_buttons);
                components
            }
            _ => vec![],
        }
    }
}
