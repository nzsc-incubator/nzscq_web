use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    phase::RechooseCharacterPhase,
    render::{
        health_display::ConstantHealthDisplay,
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{rect_button, rect_focus},
    side::Side,
};

use nzscq::choices::Character;

pub struct CharacterRechoosingPhaseRenderer<'a> {
    completion_factor: f64,
    previously_available_characters: &'a Vec<Character>,
    previously_mutually_chosen_character: Character,
    available_characters: &'a Vec<Character>,
}

impl<'a> CharacterRechoosingPhaseRenderer<'a> {
    pub fn new(phase: &'a RechooseCharacterPhase, completion_factor: f64) -> CharacterRechoosingPhaseRenderer<'a> {
        CharacterRechoosingPhaseRenderer {
            completion_factor,
            previously_available_characters: &phase.previously_available_characters,
            previously_mutually_chosen_character: phase.previously_mutually_chosen_character,
            available_characters: &phase.available_characters,
        }
    }

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

    fn human_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let index_of_chosen_character = self
                .previously_available_characters
                .iter()
                .position(|&character| character == self.previously_mutually_chosen_character)
                .expect("human should have chosen character");

            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(self.previously_mutually_chosen_character),
                    end_color: colors::character_color(self.previously_mutually_chosen_character),
                    start_shape: rect_button::background_at(index_of_chosen_character),
                    end_shape: rect_focus::left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
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

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.extend(self.health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_character);

            components
        }
    }

    fn computer_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                Component::Rect {
                    fill_color: colors::character_color(self.previously_mutually_chosen_character),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(self.previously_mutually_chosen_character),
                    end_color: colors::character_color(self.previously_mutually_chosen_character),
                    start_shape: rect_focus::far_right_background(),
                    end_shape: rect_focus::right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
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

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.extend(self.health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn pause(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |_| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                Component::Rect {
                    fill_color: colors::character_color(self.previously_mutually_chosen_character),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                Component::Rect {
                    fill_color: colors::character_color(self.previously_mutually_chosen_character),
                    shape: rect_focus::right_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
                    alpha: 1.0,
                    shape: rect_focus::right_foreground(),
                    on_click: None,
                },
            ];

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.extend(self.health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn exit(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(self.previously_mutually_chosen_character),
                    end_color: colors::character_color(self.previously_mutually_chosen_character),
                    start_shape: rect_focus::left_background(),
                    end_shape: rect_focus::far_left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
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
                    start_color: colors::character_color(self.previously_mutually_chosen_character),
                    end_color: colors::character_color(self.previously_mutually_chosen_character),
                    start_shape: rect_focus::right_background(),
                    end_shape: rect_focus::far_right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.previously_mutually_chosen_character),
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

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.extend(self.health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn rechoose_characters(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |_| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let character_buttons: Vec<Component> = self
                .available_characters
                .iter()
                .enumerate()
                .map(|(i, &character)| {
                    vec![
                        Component::Rect {
                            fill_color: colors::character_color(character),
                            shape: rect_button::background_at(i),
                            on_click: Some(Action::ChooseCharacter(character)),
                        },
                        Component::Image {
                            image_type: ImageType::Character(character),
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
            components.extend(self.health_displays());
            components
        }
    }

    fn components_displaying_characters_not_chosen_by_human(&self) -> Vec<Component> {
        let index_value_pairs_of_unchosen_characters = self
            .previously_available_characters
            .iter()
            .enumerate()
            .filter(|(_i, character)| **character != self.previously_mutually_chosen_character);

        index_value_pairs_of_unchosen_characters
            .map(|(i, &character)| {
                vec![
                    Component::Rect {
                        fill_color: colors::character_color(character),
                        shape: rect_button::background_at(i),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::Character(character),
                        alpha: 1.0,
                        shape: rect_button::foreground_at(i),

                        on_click: None,
                    },
                ]
            })
            .flatten()
            .collect()
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
            .map(|display| display.render())
            .flatten()
            .collect()
    }
}
