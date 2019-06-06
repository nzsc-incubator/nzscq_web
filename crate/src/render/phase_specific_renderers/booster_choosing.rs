use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
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
        let fade = self.fade();
        let exit = self.exit();
        let boosters = self.boosters();

        Switch5(
            (0.00..0.12, human_entrance),
            (0.12..0.24, computer_entrance),
            (0.24..0.68, fade),
            (0.68..0.80, exit),
            (0.80..=1.00, boosters),
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
                    start_shape: rect_focus::far_right_background(),
                    end_shape: rect_focus::right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(computer_character),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_focus::far_right_foreground(),
                    end_shape: rect_focus::right_foreground(),
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

    fn fade(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let human_character = self.previous_outcomes[HUMAN].0;
        let human_got_point = self.previous_outcomes[HUMAN].1 > 0;
        let computer_character = self.previous_outcomes[COMPUTER].0;
        let computer_got_point = self.previous_outcomes[COMPUTER].1 > 0;
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
            let components_displaying_human_character: Vec<Component> = {
                let lerper = lerper.sub_lerper(0.0..colors::PORTION_OF_DURATION_SPENT_FADING);
                let end_alpha = if computer_got_point { 0.0 } else { 1.0 };
                let end_color =
                    colors::character_color(&human_character).with_alpha((end_alpha * 255.0) as u8);

                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(&human_character),
                        end_color,
                        start_shape: rect_focus::left_background(),
                        end_shape: rect_focus::left_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(human_character),
                        start_alpha: 1.0,
                        end_alpha,
                        start_shape: rect_focus::left_foreground(),
                        end_shape: rect_focus::left_foreground(),
                        on_click: None,
                    },
                ]
                .into_iter()
                .map(|lerpable| lerper.lerp1(lerpable))
                .collect()
            };
            let components_displaying_computer_character: Vec<Component> = {
                let lerper = lerper.sub_lerper(0.0..colors::PORTION_OF_DURATION_SPENT_FADING);
                let end_alpha = if human_got_point { 0.0 } else { 1.0 };
                let end_color = colors::character_color(&computer_character)
                    .with_alpha((end_alpha * 255.0) as u8);

                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(&computer_character),
                        end_color,
                        start_shape: rect_focus::right_background(),
                        end_shape: rect_focus::right_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(computer_character),
                        start_alpha: 1.0,
                        end_alpha,
                        start_shape: rect_focus::right_foreground(),
                        end_shape: rect_focus::right_foreground(),
                        on_click: None,
                    },
                ]
                .into_iter()
                .map(|lerpable| lerper.lerp1(lerpable))
                .collect()
            };

            components.extend(components_displaying_characters_not_chosen_by_human);
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn exit(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let human_character = self.previous_outcomes[HUMAN].0;
        let human_got_point = self.previous_outcomes[HUMAN].1 > 0;
        let computer_character = self.previous_outcomes[COMPUTER].0;
        let computer_got_point = self.previous_outcomes[COMPUTER].1 > 0;
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
            let components_displaying_human_character: Vec<Component> = if computer_got_point {
                vec![]
            } else {
                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(&human_character),
                        end_color: colors::character_color(&human_character),
                        start_shape: rect_focus::left_background(),
                        end_shape: rect_focus::far_left_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(human_character),
                        start_alpha: 1.0,
                        end_alpha: 1.0,
                        start_shape: rect_focus::left_foreground(),
                        end_shape: rect_focus::far_left_foreground(),
                        on_click: None,
                    },
                ]
                .into_iter()
                .map(|lerpable| lerper.lerp1(lerpable))
                .collect()
            };
            let components_displaying_computer_character: Vec<Component> = if human_got_point {
                vec![]
            } else {
                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(&computer_character),
                        end_color: colors::character_color(&computer_character),
                        start_shape: rect_focus::right_background(),
                        end_shape: rect_focus::far_right_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(computer_character),
                        start_alpha: 1.0,
                        end_alpha: 1.0,
                        start_shape: rect_focus::right_foreground(),
                        end_shape: rect_focus::far_right_foreground(),
                        on_click: None,
                    },
                ]
                .into_iter()
                .map(|lerpable| lerper.lerp1(lerpable))
                .collect()
            };

            components.extend(components_displaying_characters_not_chosen_by_human);
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn boosters(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        use crate::shapes::Translate;

        let available_boosters = self.available_boosters;

        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let booster_buttons: Vec<Component> = available_boosters
                .iter()
                .enumerate()
                .map(|(i, booster)| {
                    vec![
                        LerpableComponent::Rect {
                            start_color: colors::booster_color(booster),
                            end_color: colors::booster_color(booster),
                            start_shape: rect_button::background_at(i).translate(1800.0, 0.0),
                            end_shape: rect_button::background_at(i),
                            on_click: Some(Action::ChooseBooster(*booster)),
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Booster(*booster),
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
            components.extend(booster_buttons);
            components
        }
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
