use crate::{
    click::Action,
    colors, helpers,
    paint::{Component, ImageType},
    render::{
        health_display::ConstantHealthDisplay,
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{dequeue_circle, rect_button, rect_focus},
};

use nzscq::{
    choices::{ArsenalItem, Booster, DequeueChoice},
    scoreboard::DequeueingPlayer,
};

pub struct DequeueingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previously_available_boosters: &'a Vec<Booster>,
    pub scoreboard: &'a [DequeueingPlayer; 2],
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
        move |_| {
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

    fn dequeues(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                self.human_scoreboard_display(),
                self.computer_scoreboard_display(),
            ]
            .into_iter()
            .flatten()
            .collect()
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

    fn human_scoreboard_display(&self) -> Vec<Component> {
        vec![
            self.human_pool_display(),
            self.human_entrance_and_exit_display(),
            self.human_arsenal_display(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn human_pool_display(&self) -> Vec<Component> {
        let drain_and_exit_enabled = self.available_dequeues.iter().any(|dequeue| {
            if let DequeueChoice::DrainAndExit(_) = dequeue {
                true
            } else {
                false
            }
        });

        self.human_pool()
            .iter()
            .enumerate()
            .flat_map(|(i, &arsenal_item)| {
                let row = i / 3;
                let column = i % 3;

                if drain_and_exit_enabled {
                    vec![
                        Component::Circle {
                            fill_color: colors::arsenal_item_color(arsenal_item),
                            shape: dequeue_circle::left_background_at(row, column),
                            on_click: Some(Action::ChooseDequeue(DequeueChoice::DrainAndExit(
                                arsenal_item,
                            ))),
                        },
                        Component::Image {
                            image_type: ImageType::from_arsenal_item(arsenal_item),
                            alpha: 1.0,
                            shape: dequeue_circle::left_foreground_at(row, column),
                            on_click: None,
                        },
                    ]
                } else {
                    vec![
                        Component::Circle {
                            fill_color: colors::arsenal_item_color(arsenal_item)
                                .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                            shape: dequeue_circle::left_background_at(row, column),
                            on_click: None,
                        },
                        Component::Image {
                            image_type: ImageType::from_arsenal_item(arsenal_item),
                            alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                            shape: dequeue_circle::left_foreground_at(row, column),
                            on_click: None,
                        },
                        Component::Circle {
                            fill_color: colors::OVERLAY,
                            shape: dequeue_circle::left_background_at(row, column),
                            on_click: None,
                        },
                    ]
                }
            })
            .collect()
    }

    fn human_entrance_and_exit_display(&self) -> Vec<Component> {
        let entrance = self.human().queue.entrance;
        let exit = self.human().queue.exit;
        let just_exit_enabled = self
            .available_dequeues
            .iter()
            .any(|&dequeue| DequeueChoice::JustExit == dequeue);
        let row = self.human_pool_height_in_rows();

        vec![
            entrance.map(|entering_item| {
                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(entering_item)
                            .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                        shape: dequeue_circle::left_background_at(row, 0),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::from_arsenal_item(entering_item),
                        alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                        shape: dequeue_circle::left_foreground_at(row, 0),
                        on_click: None,
                    },
                    Component::Circle {
                        fill_color: colors::OVERLAY,
                        shape: dequeue_circle::left_background_at(row, 0),
                        on_click: None,
                    },
                ]
            }),
            Some(vec![
                Component::Circle {
                    fill_color: colors::DECLINE_DEQUEUE_COLOR,
                    shape: dequeue_circle::left_background_at(row, 1),
                    on_click: Some(Action::ChooseDequeue(DequeueChoice::Decline)),
                },
                Component::Image {
                    image_type: ImageType::DeclineDequeue,
                    alpha: 1.0,
                    shape: dequeue_circle::left_foreground_at(row, 1),
                    on_click: None,
                },
            ]),
            if just_exit_enabled {
                exit.map(|exiting_item| {
                    vec![
                        Component::Circle {
                            fill_color: colors::arsenal_item_color(exiting_item),
                            shape: dequeue_circle::left_background_at(row, 2),
                            on_click: Some(Action::ChooseDequeue(DequeueChoice::JustExit)),
                        },
                        Component::Image {
                            image_type: ImageType::from_arsenal_item(exiting_item),
                            alpha: 1.0,
                            shape: dequeue_circle::left_foreground_at(row, 2),
                            on_click: None,
                        },
                    ]
                })
            } else {
                exit.map(|exiting_item| {
                    vec![
                        Component::Circle {
                            fill_color: colors::arsenal_item_color(exiting_item)
                                .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                            shape: dequeue_circle::left_background_at(row, 2),
                            on_click: None,
                        },
                        Component::Image {
                            image_type: ImageType::from_arsenal_item(exiting_item),
                            alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                            shape: dequeue_circle::left_foreground_at(row, 2),
                            on_click: None,
                        },
                        Component::Circle {
                            fill_color: colors::OVERLAY,
                            shape: dequeue_circle::left_background_at(row, 2),
                            on_click: None,
                        },
                    ]
                })
            },
        ]
        .into_iter()
        .flatten()
        .flatten()
        .collect()
    }

    fn human_arsenal_display(&self) -> Vec<Component> {
        let row_offset = self.human_pool_height_in_rows() + 1;

        self.human()
            .arsenal
            .iter()
            .enumerate()
            .flat_map(|(i, &arsenal_item)| {
                let row = i / 3;
                let column = i % 3;
                let row = row + row_offset;

                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(arsenal_item)
                            .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                        shape: dequeue_circle::left_background_at(row, column),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::from_arsenal_item(arsenal_item),
                        alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                        shape: dequeue_circle::left_foreground_at(row, column),
                        on_click: None,
                    },
                    Component::Circle {
                        fill_color: colors::OVERLAY,
                        shape: dequeue_circle::left_background_at(row, column),
                        on_click: None,
                    },
                ]
            })
            .collect()
    }

    fn computer_scoreboard_display(&self) -> Vec<Component> {
        vec![
            self.computer_pool_display(),
            self.computer_entrance_and_exit_display(),
            self.computer_arsenal_display(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn computer_pool_display(&self) -> Vec<Component> {
        // TODO
        vec![]
    }

    fn computer_entrance_and_exit_display(&self) -> Vec<Component> {
        // TODO
        vec![]
    }

    fn computer_arsenal_display(&self) -> Vec<Component> {
        // TODO
        vec![]
    }

    fn human_pool_height_in_rows(&self) -> usize {
        (self.human_pool().len() + 2) / 3
    }

    fn human_pool(&self) -> &Vec<ArsenalItem> {
        &self.human().queue.pool
    }

    fn human_booster(&self) -> Booster {
        self.human().booster
    }

    fn computer_booster(&self) -> Booster {
        self.computer().booster
    }

    fn health_display(&self) -> Vec<Component> {
        ConstantHealthDisplay {
            human_health: helpers::opponent_points_to_own_health(self.computer().points),
            computer_health: helpers::opponent_points_to_own_health(self.human().points),
        }
        .render()
    }

    fn human(&self) -> &DequeueingPlayer {
        &self.scoreboard[HUMAN]
    }

    fn computer(&self) -> &DequeueingPlayer {
        &self.scoreboard[COMPUTER]
    }
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
