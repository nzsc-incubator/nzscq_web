use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    phase::ChooseCharacterPhase,
    render::{
        health_display::ConstantHealthDisplay,
        lerp::{LerpableComponent, Lerper},
        Render,
    },
    shapes::rect_button,
    side::Side,
    transform::Translate,
};

use nzscq::choices::Character;

pub struct CharacterChoosingPhaseRenderer<'a> {
    available_characters: &'a Vec<Character>,
}

impl<'a> CharacterChoosingPhaseRenderer<'a> {
    pub fn new(phase: &'a ChooseCharacterPhase) -> CharacterChoosingPhaseRenderer<'a> {
        CharacterChoosingPhaseRenderer {
            available_characters: &phase.available_characters,
        }
    }

    fn health_displays(&self) -> Vec<Component> {
        let human_display = ConstantHealthDisplay {
            side: Side::Left,
            health: 5,
        };
        let computer_display = ConstantHealthDisplay {
            side: Side::Right,
            health: 5,
        };

        vec![human_display, computer_display]
            .into_iter()
            .map(|display| display.render(()))
            .flatten()
            .collect()
    }
}

impl<'a> Render<f64> for CharacterChoosingPhaseRenderer<'a> {
    fn render(&self, completion_factor: f64) -> Vec<Component> {
        let lerper = Lerper::from_completion_factor(completion_factor);
        let mut components = vec![Component::Background {
            color: colors::BACKGROUND,
        }];
        let character_buttons: Vec<Component> = self
            .available_characters
            .iter()
            .enumerate()
            .map(|(i, &character)| {
                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(character),
                        end_color: colors::character_color(character),
                        start_shape: rect_button::background_at(i).translate(1800.0, 0.0),
                        end_shape: rect_button::background_at(i),
                        on_click: Some(Action::ChooseCharacter(character)),
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(character),
                        start_alpha: 1.0,
                        end_alpha: 1.0,
                        start_shape: rect_button::foreground_at(i).translate(1800.0, 0.0),
                        end_shape: rect_button::foreground_at(i),
                        on_click: None,
                    },
                ]
                .into_iter()
            })
            .flatten()
            .map(|lerpable| lerper.lerp1(lerpable))
            .collect();
        components.extend(character_buttons);
        components.extend(self.health_displays());
        components
    }
}
