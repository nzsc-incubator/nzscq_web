use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        heart,
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
    },
    shapes::{rect_button, rect_focus},
};

use nzscq::choices::Character;

pub struct CharacterRechoosingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previously_available_characters: &'a Vec<Character>,
    pub previously_mutually_chosen_character: Character,
    pub available_characters: &'a Vec<Character>,
}

impl<'a> CharacterRechoosingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let human_entrance = self.human_entrance();
        let computer_entrance = self.computer_entrance();
        let pause = self.pause();
        let exit = self.exit();
        let rechoose_characters = self.rechoose_characters();

        Switch5(
            (0.00..0.15, human_entrance),
            (0.15..0.30, computer_entrance),
            (0.30..0.85, pause),
            (0.85..1.00, exit),
            (1.00..=1.00, rechoose_characters),
        )
        .case(self.completion_factor)
        .expect("should have legal completion range")
    }

    fn human_entrance(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let mutual_character = self.previously_mutually_chosen_character;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != mutual_character);
            let index_of_chosen_character = previously_available_characters
                .iter()
                .position(|character| *character == mutual_character)
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
                    start_color: colors::character_color(&mutual_character),
                    end_color: colors::character_color(&mutual_character),
                    start_shape: rect_button::background_at(index_of_chosen_character),
                    end_shape: rect_focus::left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(mutual_character),
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
        let mutual_character = self.previously_mutually_chosen_character;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != mutual_character);

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
                    fill_color: colors::character_color(&mutual_character),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(mutual_character),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(&mutual_character),
                    end_color: colors::character_color(&mutual_character),
                    start_shape: rect_focus::far_right_background(),
                    end_shape: rect_focus::right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(mutual_character),
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

    fn pause(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let mutual_character = self.previously_mutually_chosen_character;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != mutual_character);

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
                    fill_color: colors::character_color(&mutual_character),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(mutual_character),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                Component::Rect {
                    fill_color: colors::character_color(&mutual_character),
                    shape: rect_focus::right_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(mutual_character),
                    alpha: 1.0,
                    shape: rect_focus::right_foreground(),
                    on_click: None,
                },
            ];

            components.extend(components_displaying_characters_not_chosen_by_human);
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn exit(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        let mutual_character = self.previously_mutually_chosen_character;
        let previously_available_characters = self.previously_available_characters;

        move |lerper| {
            let index_value_pairs_of_unchosen_characters = previously_available_characters
                .iter()
                .enumerate()
                .filter(|(_i, character)| **character != mutual_character);

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
                LerpableComponent::Rect {
                    start_color: colors::character_color(&mutual_character),
                    end_color: colors::character_color(&mutual_character),
                    start_shape: rect_focus::left_background(),
                    end_shape: rect_focus::far_left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(mutual_character),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_focus::left_foreground(),
                    end_shape: rect_focus::far_left_foreground(),
                    on_click: None,
                },
            ]
            .into_iter()
            .map(|lerpable| lerper.lerp1(lerpable))
            .collect();
            let components_displaying_computer_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(&mutual_character),
                    end_color: colors::character_color(&mutual_character),
                    start_shape: rect_focus::right_background(),
                    end_shape: rect_focus::far_right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(mutual_character),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_focus::right_foreground(),
                    end_shape: rect_focus::far_right_foreground(),
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

    fn rechoose_characters(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        use crate::shapes::Translate;

        let available_characters = self.available_characters;

        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
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
            let character_buttons: Vec<Component> = available_characters
                .iter()
                .enumerate()
                .map(|(i, character)| {
                    vec![
                        Component::Rect {
                            fill_color: colors::character_color(character),
                            shape: rect_button::background_at(i),
                            on_click: Some(Action::ChooseCharacter(*character)),
                        },
                        Component::Image {
                            image_type: ImageType::Character(*character),
                            alpha: 1.0,
                            shape: rect_button::foreground_at(i),
                            on_click: None,
                        },
                    ]
                    .into_iter()
                })
                .flatten()
                .collect();
            components.extend(character_buttons);
            components.extend(trapezoids);
            components.extend(human_hearts);
            components.extend(computer_hearts);
            components
        }
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;