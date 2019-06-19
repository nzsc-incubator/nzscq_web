use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        Render,
        health_display::{ConstantHealthDisplay, FadingHealthDisplay},
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
    pub previous_outcome: &'a Vec<CharacterHeadstart>,
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

    fn human_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let index_of_chosen_character = self
                .previously_available_characters
                .iter()
                .position(|&character| character == self.human_character())
                .expect("human should have chosen character");

            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(self.human_character()),
                    end_color: colors::character_color(self.human_character()),
                    start_shape: rect_button::background_at(index_of_chosen_character),
                    end_shape: rect_focus::left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.human_character()),
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
            components.push(overlay);
            components.extend(self.previous_health_display());
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
            let components_displaying_human_character = vec![
                Component::Rect {
                    fill_color: colors::character_color(self.human_character()),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Character(self.human_character()),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_character: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::character_color(self.computer_character()),
                    end_color: colors::character_color(self.computer_character()),
                    start_shape: rect_focus::far_right_background(),
                    end_shape: rect_focus::right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Character(self.computer_character()),
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
            components.push(overlay);
            components.extend(self.previous_health_display());
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn fade(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_character: Vec<Component> = {
                let lerper = lerper.sub_lerper(0.0..colors::PORTION_OF_DURATION_SPENT_FADING);
                let end_alpha = if self.did_computer_get_point() {
                    0.0
                } else {
                    1.0
                };
                let end_color = colors::character_color(self.human_character())
                    .with_alpha((end_alpha * 255.0) as u8);

                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(self.human_character()),
                        end_color,
                        start_shape: rect_focus::left_background(),
                        end_shape: rect_focus::left_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(self.human_character()),
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
                let end_alpha = if self.did_human_get_point() { 0.0 } else { 1.0 };
                let end_color = colors::character_color(self.computer_character())
                    .with_alpha((end_alpha * 255.0) as u8);

                vec![
                    LerpableComponent::Rect {
                        start_color: colors::character_color(self.computer_character()),
                        end_color,
                        start_shape: rect_focus::right_background(),
                        end_shape: rect_focus::right_background(),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Character(self.computer_character()),
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

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.push(overlay);
            components.extend(lerper.lerp1(self.fading_health_display()));
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
            let components_displaying_human_character: Vec<Component> =
                if self.did_computer_get_point() {
                    vec![]
                } else {
                    vec![
                        LerpableComponent::Rect {
                            start_color: colors::character_color(self.human_character()),
                            end_color: colors::character_color(self.human_character()),
                            start_shape: rect_focus::left_background(),
                            end_shape: rect_focus::far_left_background(),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Character(self.human_character()),
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
            let components_displaying_computer_character: Vec<Component> =
                if self.did_human_get_point() {
                    vec![]
                } else {
                    vec![
                        LerpableComponent::Rect {
                            start_color: colors::character_color(self.computer_character()),
                            end_color: colors::character_color(self.computer_character()),
                            start_shape: rect_focus::right_background(),
                            end_shape: rect_focus::far_right_background(),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Character(self.computer_character()),
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

            components.extend(self.components_displaying_characters_not_chosen_by_human());
            components.push(overlay);
            components.extend(self.current_health_display());
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn boosters(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        use crate::shapes::Translate;

        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let booster_buttons: Vec<Component> = self
                .available_boosters
                .iter()
                .enumerate()
                .map(|(i, &booster)| {
                    vec![
                        LerpableComponent::Rect {
                            start_color: colors::booster_color(booster),
                            end_color: colors::booster_color(booster),
                            start_shape: rect_button::background_at(i).translate(1800.0, 0.0),
                            end_shape: rect_button::background_at(i),
                            on_click: Some(Action::ChooseBooster(booster)),
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Booster(booster),
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
            components.extend(self.current_health_display());
            components
        }
    }

    fn components_displaying_characters_not_chosen_by_human(&self) -> Vec<Component> {
        let index_value_pairs_of_unchosen_characters = self
            .previously_available_characters
            .iter()
            .enumerate()
            .filter(|(_i, character)| **character != self.human_character());

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

    fn human_character(&self) -> Character {
        self.previous_outcome[HUMAN].0
    }

    fn computer_character(&self) -> Character {
        self.previous_outcome[COMPUTER].0
    }

    fn previous_health_display(&self) -> Vec<Component> {
        ConstantHealthDisplay {
            human_health: self.previous_human_health(),
            computer_health: self.previous_computer_health(),
        }
        .render()
    }

    fn fading_health_display(&self) -> FadingHealthDisplay {
        FadingHealthDisplay {
            previous_human_health: self.previous_human_health(),
            previous_computer_health: self.previous_computer_health(),
            is_human_losing_a_heart: self.did_computer_get_point(),
            is_computer_losing_a_heart: self.did_human_get_point(),
        }
    }

    fn current_health_display(&self) -> Vec<Component> {
        ConstantHealthDisplay {
            human_health: self.human_health(),
            computer_health: self.computer_health(),
        }
        .render()
    }

    fn human_health(&self) -> u8 {
        if self.did_computer_get_point() {
            4
        } else {
            5
        }
    }

    fn did_computer_get_point(&self) -> bool {
        self.previous_outcome[COMPUTER].1 > 0
    }

    fn computer_health(&self) -> u8 {
        if self.did_human_get_point() {
            4
        } else {
            5
        }
    }

    fn did_human_get_point(&self) -> bool {
        self.previous_outcome[HUMAN].1 > 0
    }

    fn previous_human_health(&self) -> u8 {
        5
    }

    fn previous_computer_health(&self) -> u8 {
        5
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
