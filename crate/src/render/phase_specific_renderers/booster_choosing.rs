use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    phase::ChooseBoosterPhase,
    render::{
        health_display::{ConstantHealthDisplay, FadingHealthDisplay},
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{rect_button, rect_focus},
    side::Side,
};

use nzscq::{
    choices::{Booster, Character},
    outcomes::CharacterHeadstart,
};

pub struct BoosterChoosingPhaseRenderer<'a> {
    previously_available_characters: &'a Vec<Character>,
    previous_outcome: &'a Vec<CharacterHeadstart>,
    available_boosters: &'a Vec<Booster>,
}

impl<'a> BoosterChoosingPhaseRenderer<'a> {
    pub fn new(
        phase: &'a ChooseBoosterPhase,
    ) -> BoosterChoosingPhaseRenderer<'a> {
        BoosterChoosingPhaseRenderer {
            previously_available_characters: &phase.previously_available_characters,
            previous_outcome: &phase.previous_outcome,
            available_boosters: &phase.available_boosters,
        }
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
            components.extend(self.previous_health_displays());
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
            components.extend(self.previous_health_displays());
            components.push(overlay);
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
            components.extend(self.fade_case_non_fading_health_displays());
            components.push(overlay);
            components.extend(self.fade_case_fading_health_displays(&lerper));
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
            components.extend(self.current_health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_character);
            components.extend(components_displaying_computer_character);

            components
        }
    }

    fn boosters(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        use crate::transform::Translate;

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
            components.extend(self.current_health_displays());
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

    fn previous_health_displays(&self) -> Vec<Component> {
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

    fn fade_case_fading_health_displays(&self, lerper: &Lerper) -> Vec<Component> {
        let human_components = if self.did_computer_get_point() {
            lerper.lerp1(FadingHealthDisplay {
                side: Side::Left,
                starting_health: 5,
            })
        } else {
            vec![]
        };
        let computer_components = if self.did_human_get_point() {
            lerper.lerp1(FadingHealthDisplay {
                side: Side::Right,
                starting_health: 5,
            })
        } else {
            vec![]
        };

        vec![human_components, computer_components]
            .into_iter()
            .flatten()
            .collect()
    }

    fn fade_case_non_fading_health_displays(&self) -> Vec<Component> {
        let human_components = if self.did_computer_get_point() {
            vec![]
        } else {
            ConstantHealthDisplay {
                side: Side::Left,
                health: 5,
            }
            .render(())
        };
        let computer_components = if self.did_human_get_point() {
            vec![]
        } else {
            ConstantHealthDisplay {
                side: Side::Right,
                health: 5,
            }
            .render(())
        };

        vec![human_components, computer_components]
            .into_iter()
            .flatten()
            .collect()
    }

    fn current_health_displays(&self) -> Vec<Component> {
        let human_display = ConstantHealthDisplay {
            side: Side::Left,
            health: self.human_health(),
        };
        let computer_display = ConstantHealthDisplay {
            side: Side::Right,
            health: self.computer_health(),
        };

        vec![human_display, computer_display]
            .into_iter()
            .map(|display| display.render(()))
            .flatten()
            .collect()
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
}

impl<'a> Render<f64> for BoosterChoosingPhaseRenderer<'a> {
    fn render(&self, completion_factor: f64) -> Vec<Component> {
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
        .case(completion_factor)
        .expect("should have legal completion range")
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
