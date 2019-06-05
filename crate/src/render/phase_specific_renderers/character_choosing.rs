use crate::{
    click::Action,
    paint::{Component, ImageType},
    render::colors,
    render::lerp::{LerpInto, Lerpable, Lerper},
    shapes::{rect_button, Translate},
};

use nzscq::choices::Character;

pub struct CharacterChoosingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub characters: &'a Vec<Character>,
}

impl<'a> CharacterChoosingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let lerper = Lerper::from_completion_factor(self.completion_factor);
        let mut components = vec![Component::Background(colors::BACKGROUND)];
        let character_buttons: Vec<Component> = self
            .characters
            .iter()
            .enumerate()
            .map(|(i, character)| {
                vec![
                    Lerpable::Rect {
                        fill_color: colors::character_color(character),
                        start: rect_button::background_at(i).translate(1800.0, 0.0),
                        end: rect_button::background_at(i),
                        on_click: Some(Action::ChooseCharacter(*character)),
                    },
                    Lerpable::Image {
                        image_type: ImageType::Character(*character),
                        start: rect_button::foreground_at(i).translate(1800.0, 0.0),
                        end: rect_button::foreground_at(i),
                        on_click: None,
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
}
