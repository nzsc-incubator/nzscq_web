use crate::{
    click::Action,
    colors,
    paint::{Component, ImageType},
    render::{
        heart::{ConstantHealthDisplay, FadingHealthDisplay},
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
    },
    shapes::{rect_button, rect_focus},
};

use nzscq::choices::{Booster, DequeueChoice};

pub struct DequeueingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previously_available_boosters: &'a Vec<Booster>,
    pub previous_outcome: &'a Vec<Booster>,
    pub health: &'a Vec<u8>,
    pub available_dequeues: &'a Vec<DequeueChoice>,
}

impl<'a> DequeueingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let human_entrance = self.human_entrance();
        let computer_entrance = self.computer_entrance();
        let pause = self.pause();
        let exit = self.exit();
        let dequeues = self.dequeues();

        Switch5(
            (0.00..0.12, human_entrance),
            (0.12..0.24, computer_entrance),
            (0.24..0.68, pause),
            (0.68..0.80, exit),
            (0.80..=1.00, dequeues),
        )
        .case(self.completion_factor)
        .expect("should have legal completion range")
    }

    fn human_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let index_of_chosen_booster = self
                .previously_available_boosters
                .iter()
                .position(|&booster| booster == self.human_booster())
                .expect("human should have chosen booster");

            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_booster: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::booster_color(self.human_booster()),
                    end_color: colors::booster_color(self.human_booster()),
                    start_shape: rect_button::background_at(index_of_chosen_booster),
                    end_shape: rect_focus::left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Booster(self.human_booster()),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: rect_button::foreground_at(index_of_chosen_booster),
                    end_shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ]
            .into_iter()
            .map(|lerpable| lerper.lerp1(lerpable))
            .collect();

            components.extend(self.components_displaying_boosters_not_chosen_by_human());
            components.push(overlay);
            components.extend(self.health_display());
            components.extend(components_displaying_human_booster);

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
            let components_displaying_human_booster = vec![
                Component::Rect {
                    fill_color: colors::booster_color(self.human_booster()),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Booster(self.human_booster()),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_booster: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::booster_color(self.computer_booster()),
                    end_color: colors::booster_color(self.computer_booster()),
                    start_shape: rect_focus::far_right_background(),
                    end_shape: rect_focus::right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Booster(self.computer_booster()),
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

            components.extend(self.components_displaying_boosters_not_chosen_by_human());
            components.push(overlay);
            components.extend(self.health_display());
            components.extend(components_displaying_human_booster);
            components.extend(components_displaying_computer_booster);

            components
        }
    }

    fn pause(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            let mut components = vec![Component::Background {
                color: colors::BACKGROUND,
            }];
            let overlay = Component::Background {
                color: colors::OVERLAY,
            };
            let components_displaying_human_booster = vec![
                Component::Rect {
                    fill_color: colors::booster_color(self.human_booster()),
                    shape: rect_focus::left_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Booster(self.human_booster()),
                    alpha: 1.0,
                    shape: rect_focus::left_foreground(),
                    on_click: None,
                },
            ];
            let components_displaying_computer_booster = vec![
                Component::Rect {
                    fill_color: colors::booster_color(self.computer_booster()),
                    shape: rect_focus::right_background(),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::Booster(self.computer_booster()),
                    alpha: 1.0,
                    shape: rect_focus::right_foreground(),
                    on_click: None,
                },
            ];

            components.extend(self.components_displaying_boosters_not_chosen_by_human());
            components.push(overlay);
            components.extend(self.health_display());
            components.extend(components_displaying_human_booster);
            components.extend(components_displaying_computer_booster);

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
            let components_displaying_human_booster: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::booster_color(self.human_booster()),
                    end_color: colors::booster_color(self.human_booster()),
                    start_shape: rect_focus::left_background(),
                    end_shape: rect_focus::far_left_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Booster(self.human_booster()),
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
            let components_displaying_computer_booster: Vec<Component> = vec![
                LerpableComponent::Rect {
                    start_color: colors::booster_color(self.computer_booster()),
                    end_color: colors::booster_color(self.computer_booster()),
                    start_shape: rect_focus::right_background(),
                    end_shape: rect_focus::far_right_background(),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::Booster(self.computer_booster()),
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

            components.extend(self.components_displaying_boosters_not_chosen_by_human());
            components.push(overlay);
            components.extend(self.health_display());
            components.extend(components_displaying_human_booster);
            components.extend(components_displaying_computer_booster);

            components
        }
    }

    fn dequeues(&self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        use crate::shapes::Translate;


        move |lerper| {
            // let mut components = vec![Component::Background {
            //     color: colors::BACKGROUND,
            // }];
            // let booster_buttons: Vec<Component> = available_boosters
            //     .iter()
            //     .enumerate()
            //     .map(|(i, booster)| {
            //         vec![
            //             LerpableComponent::Rect {
            //                 start_color: colors::booster_color(booster),
            //                 end_color: colors::booster_color(booster),
            //                 start_shape: rect_button::background_at(i).translate(1800.0, 0.0),
            //                 end_shape: rect_button::background_at(i),
            //                 on_click: Some(Action::ChooseBooster(booster)),
            //             },
            //             LerpableComponent::Image {
            //                 image_type: ImageType::Booster(booster),
            //                 start_alpha: 1.0,
            //                 end_alpha: 1.0,
            //                 start_shape: rect_button::foreground_at(i).translate(1800.0, 0.0),
            //                 end_shape: rect_button::foreground_at(i),
            //                 on_click: None,
            //             },
            //         ]
            //         .into_iter()
            //     })
            //     .flatten()
            //     .map(|lerpable| lerper.lerp1(lerpable))
            //     .collect();
            // components.extend(booster_buttons);
            // components.extend(self.health_display());
            // components
            vec![Component::Background {
                color: colors::BACKGROUND,
            }]
        }
    }

    fn components_displaying_boosters_not_chosen_by_human(&self) -> Vec<Component> {
        let index_value_pairs_of_unchosen_boosters = self
            .previously_available_boosters
            .iter()
            .enumerate()
            .filter(|(_i, &booster)| booster != self.human_booster());

        index_value_pairs_of_unchosen_boosters
            .map(|(i, &booster)| {
                vec![
                    Component::Rect {
                        fill_color: colors::booster_color(booster),
                        shape: rect_button::background_at(i),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::Booster(booster),
                        alpha: 1.0,
                        shape: rect_button::foreground_at(i),

                        on_click: None,
                    },
                ]
            })
            .flatten()
            .collect()
    }

    fn human_booster(&self) -> Booster {
        self.previous_outcome[HUMAN]
    }

    fn computer_booster(&self) -> Booster {
        self.previous_outcome[COMPUTER]
    }

    fn health_display(&self) -> Vec<Component> {
        ConstantHealthDisplay {
            human_health: self.health[HUMAN],
            computer_health: self.health[COMPUTER],
        }
        .into()
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
