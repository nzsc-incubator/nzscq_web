use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch4},
    },
    shapes::{rect_button, rect_focus},
};

use nzscq::{
    choices::{Booster, Character},
    outcomes::CharacterHeadstart,
};

pub struct BoosterChoosingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previously_available_characters: &'a Vec<Character>,
    pub previous_outcomes: &'a Vec<CharacterHeadstart>,
    pub available_boosters: &'a Vec<Booster>,
}

impl<'a> BoosterChoosingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let human_entrance = self.human_entrance();
        let computer_entrance = self.computer_entrance();
        let fade = |_| vec![];
        let exit = |_| vec![];

        Switch4(
            (0.0..0.15, human_entrance),
            (0.15..0.30, computer_entrance),
            (0.30..0.85, fade),
            (0.85..=1.0, exit),
        )
        .case(self.completion_factor)
        .expect("should have legal completion range")
    }

    fn human_entrance(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let human_character = self.previous_outcomes[HUMAN].0;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != human_character);
            let index_of_chosen_character = previously_available_characters
                .iter()
                .position(|character| *character == human_character)
                .expect("human should have chosen character");

            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let components_dipslaying_characters_not_chosen_by_human: Vec<Component> =
                index_value_pairs_of_unchosen_characters
                    .map(|(i, character)| {
                        vec![
                            Component::Rect {
                                fill_color: colors::character_color(character),
                                shape: rect_button::background_at(i),
                                on_click: None,
                            },
                            Component::Image {
                                image_type: ImageType::Character(*character),
                                alpha: 1.0,
                                shape: rect_button::foreground_at(i),

                                on_click: None,
                            },
                        ]
                    })
                    .flatten()
                    .collect();
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(&human_character),
                    end_color: colors::character_color(&human_character),
                    start_shape: rect_button::background_at(index_of_chosen_character),
                    end_shape: rect_focus::left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(human_character),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_button::foreground_at(index_of_chosen_character),
                    end_shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ]
            .into_iter()
            .map(|lerpable| lerper.lerp1(lerpable))
            .collect();

            components.extend(components_dipslaying_characters_not_chosen_by_human);
            components.push(overlay);
            components.extend(components_displaying_human_character);

            components
        }
    }

    fn computer_entrance(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let human_character = self.previous_outcomes[HUMAN].0;
        let computer_character = self.previous_outcomes[COMPUTER].0;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != human_character);

            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let components_displaying_characters_not_chosen_by_human: Vec<Component> =
                index_value_pairs_of_unchosen_characters
                    .map(|(i, character)| {
                        vec![
                            Component::Rect {
                                fill_color: colors::character_color(character),
                                shape: rect_button::background_at(i),
                                on_click: None,
                            },
                            Component::Image {
                                image_type: ImageType::Character(*character),
                                alpha: 1.0,
                                shape: rect_button::foreground_at(i),

                                on_click: None,
                            },
                        ]
                    })
                    .flatten()
                    .collect();
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                Component::Rect {
                    fill_color: colors::character_color(&human_character),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(human_character),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(&computer_character),
                    end_color: colors::character_color(&computer_character),
                    start_shape: rect_focus::right_initial_background(),
                    end_shape: rect_focus::right_final_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(computer_character),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_focus::right_initial_foreground(),
                    end_shape: rect_focus::right_final_foreground(),
                    on_click: None,
                },
            ]
            .into_iter()
            .map(|lerpable| lerper.lerp1(lerpable))
            .collect();

            components.extend(components_displaying_characters_not_chosen_by_human);
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
