use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        heart,
        lerp::{LerpableComponent, Lerper},
        switch::Switch,
    },
    shapes::{rect_button, Translate},
};

use nzscq::choices::Character;

pub struct CharacterChoosingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub available_characters: &'a Vec<Character>,
}

impl<'a> CharacterChoosingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let lerper = Lerper::from_completion_factor(self.completion_factor);
        let mut components = vec![Component::Background {
            color: colors::BACKGROUND,
        }];
        let character_buttons: Vec<Component> = self
            .available_characters
            .iter()
            .enumerate()
            .map(|(i, character)| {
                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(character),
                        end_color: colors::character_color(character),
                        start_shape: rect_button::background_at(i).translate(1800.0, 0.0),
                        end_shape: rect_button::background_at(i),
                        on_click: Some(Action::ChooseCharacter(*character)),
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(*character),
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
        let trapezoids = vec![
            Component::HealthTrapezoid {
                x: 20.0,
                y: 15.0,
                border_width: colors::TRAPEZOID_BORDER_WIDTH,
                border_color: colors::TRAPEZOID_BORDER,
                fill_color: colors::TRAPEZOID_FILL,
            },
            Component::HealthTrapezoid {
                x: 1340.0,
                y: 15.0,
                border_width: colors::TRAPEZOID_BORDER_WIDTH,
                border_color: colors::TRAPEZOID_BORDER,
                fill_color: colors::TRAPEZOID_FILL,
            },
        ];
        let human_hearts: Vec<Component> = (0..5)
            .into_iter()
            .map(|i| heart::left_at(i).case(0.0).expect("should find a case"))
            .flatten()
            .collect();
        let computer_hearts: Vec<Component> = (0..5)
            .into_iter()
            .map(|i| heart::right_at(i).case(0.0).expect("should find a case"))
            .flatten()
            .collect();
        components.extend(character_buttons);
        components.extend(trapezoids);
        components.extend(human_hearts);
        components.extend(computer_hearts);
        components
    }
}
