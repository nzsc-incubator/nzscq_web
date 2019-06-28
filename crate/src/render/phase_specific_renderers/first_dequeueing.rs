use crate::{
    click::Action,
    colors, helpers,
    paint::{Component, ImageType},
    render::{
        arrow,
        health_display::ConstantHealthDisplay,
        lerp::{LerpableComponent, Lerper},
        pill::Pill,
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{
        dequeue_circle::{self, CirclePosition},
        rect_button, rect_focus,
    },
    side::Side,
    transform::Translate,
};

use nzscq::{
    choices::{Booster, DequeueChoice},
    scoreboard::DequeueingPlayer,
};

pub struct FirstDequeueingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previously_available_boosters: &'a Vec<Booster>,
    pub scoreboard: &'a [DequeueingPlayer; 2],
    pub available_dequeues: &'a [Vec<DequeueChoice>; 2],
}

impl<'a> FirstDequeueingPhaseRenderer<'a> {
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
            components.extend(self.health_displays());
            components.push(overlay);
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
            components.extend(self.health_displays());
            components.push(overlay);
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
            components.extend(self.health_displays());
            components.push(overlay);
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
            components.extend(self.health_displays());
            components.push(overlay);
            components.extend(components_displaying_human_booster);
            components.extend(components_displaying_computer_booster);

            components
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
        self.scoreboard[HUMAN].booster
    }

    fn computer_booster(&self) -> Booster {
        self.scoreboard[COMPUTER].booster
    }

    fn dequeues(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_displays(),
                self.human_scoreboard_display()
                    .into_iter()
                    .map(|component| component.translate(lerper.lerp(-553.2, 0.0), 0.0))
                    .collect(),
                self.computer_scoreboard_display()
                    .into_iter()
                    .map(|component| component.translate(lerper.lerp(553.2, 0.0), 0.0))
                    .collect(),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn human_scoreboard_display(&self) -> Vec<Component> {
        vec![
            pool_display(self.human()),
            entrance_and_exit_display(self.human()),
            arsenal_display(self.human()),
            arrows(self.human()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn computer_scoreboard_display(&self) -> Vec<Component> {
        vec![
            pool_display(self.computer()),
            entrance_and_exit_display(self.computer()),
            arsenal_display(self.computer()),
            arrows(self.computer()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn health_displays(&self) -> Vec<Component> {
        let human_display = ConstantHealthDisplay {
            side: Side::Left,
            health: helpers::opponent_points_to_own_health(self.computer_points()),
        };
        let computer_display = ConstantHealthDisplay {
            side: Side::Right,
            health: helpers::opponent_points_to_own_health(self.human_points()),
        };

        vec![human_display, computer_display]
            .into_iter()
            .map(|display| display.render())
            .flatten()
            .collect()
    }

    fn human_points(&self) -> u8 {
        self.scoreboard[HUMAN].points
    }

    fn computer_points(&self) -> u8 {
        self.scoreboard[COMPUTER].points
    }

    fn human(&self) -> ScoreboardRenderArgs {
        ScoreboardRenderArgs {
            player: &self.scoreboard[HUMAN],
            side: Side::Left,
            dequeues: &self.available_dequeues[HUMAN],
        }
    }

    fn computer(&self) -> ScoreboardRenderArgs {
        ScoreboardRenderArgs {
            player: &self.scoreboard[COMPUTER],
            side: Side::Right,
            dequeues: &self.available_dequeues[COMPUTER],
        }
    }
}

fn pool_display(args: ScoreboardRenderArgs) -> Vec<Component> {
    let ScoreboardRenderArgs {
        player,
        side,
        dequeues,
    } = args;

    let drain_and_exit_enabled = dequeues.iter().any(|dequeue| {
        if let DequeueChoice::DrainAndExit(_) = dequeue {
            true
        } else {
            false
        }
    });

    let pill = Pill {
        position: CirclePosition {
            from: side,
            column: 0,
            row: 0,
        },
        width_in_columns: 3,
        height_in_rows: helpers::height_in_rows(&player.queue.pool, 3),
        enabled: drain_and_exit_enabled,
    };

    let pool = player
        .queue
        .pool
        .iter()
        .enumerate()
        .flat_map(|(i, &arsenal_item)| {
            let row = i / 3;
            let column = i % 3;

            if drain_and_exit_enabled {
                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(arsenal_item),
                        shape: dequeue_circle::background_at(side, row, column),
                        on_click: side.if_left(Action::ChooseDequeue(DequeueChoice::DrainAndExit(
                            arsenal_item,
                        ))),
                    },
                    Component::Image {
                        image_type: ImageType::from(arsenal_item),
                        alpha: 1.0,
                        shape: dequeue_circle::foreground_at(side, row, column),
                        on_click: None,
                    },
                ]
            } else {
                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(arsenal_item)
                            .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                        shape: dequeue_circle::background_at(side, row, column),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::from(arsenal_item),
                        alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                        shape: dequeue_circle::foreground_at(side, row, column),
                        on_click: None,
                    },
                    Component::Circle {
                        fill_color: colors::OVERLAY,
                        shape: dequeue_circle::background_at(side, row, column),
                        on_click: None,
                    },
                ]
            }
        });

    pill.render().into_iter().chain(pool).collect()
}

fn entrance_and_exit_display(args: ScoreboardRenderArgs) -> Vec<Component> {
    let ScoreboardRenderArgs {
        player,
        side,
        dequeues,
    } = args;

    let entrance = player.queue.entrance;
    let exit = player.queue.exit;
    let just_exit_enabled = dequeues
        .iter()
        .any(|&dequeue| DequeueChoice::JustExit == dequeue);
    let pool_height_in_rows = helpers::height_in_rows(&player.queue.pool, 3);
    let row = pool_height_in_rows;

    let background_pill = Pill {
        position: CirclePosition {
            from: side,
            column: 0,
            row: pool_height_in_rows,
        },
        width_in_columns: 3,
        height_in_rows: 1,
        enabled: false,
    };
    let decline_and_exit_pill = Pill {
        position: CirclePosition {
            from: side,
            column: 1,
            row: pool_height_in_rows,
        },
        width_in_columns: 2,
        height_in_rows: 1,
        enabled: true,
    };

    vec![
        Some(background_pill.render()),
        Some(decline_and_exit_pill.render()),
        entrance.map(|entering_item| {
            vec![
                Component::Circle {
                    fill_color: colors::arsenal_item_color(entering_item)
                        .with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA),
                    shape: dequeue_circle::background_at(side, row, 0),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::from(entering_item),
                    alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                    shape: dequeue_circle::foreground_at(side, row, 0),
                    on_click: None,
                },
                Component::Circle {
                    fill_color: colors::OVERLAY,
                    shape: dequeue_circle::background_at(side, row, 0),
                    on_click: None,
                },
            ]
        }),
        Some(vec![
            Component::Circle {
                fill_color: colors::DECLINE_DEQUEUE_COLOR,
                shape: dequeue_circle::background_at(side, row, 1),
                on_click: side.if_left(Action::ChooseDequeue(DequeueChoice::Decline)),
            },
            Component::Image {
                image_type: ImageType::DeclineDequeue,
                alpha: 1.0,
                shape: dequeue_circle::foreground_at(side, row, 1),
                on_click: None,
            },
        ]),
        if just_exit_enabled {
            exit.map(|exiting_item| {
                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(exiting_item),
                        shape: dequeue_circle::background_at(side, row, 2),
                        on_click: side.if_left(Action::ChooseDequeue(DequeueChoice::JustExit)),
                    },
                    Component::Image {
                        image_type: ImageType::from(exiting_item),
                        alpha: 1.0,
                        shape: dequeue_circle::foreground_at(side, row, 2),
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
                        shape: dequeue_circle::background_at(side, row, 2),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::from(exiting_item),
                        alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                        shape: dequeue_circle::foreground_at(side, row, 2),
                        on_click: None,
                    },
                    Component::Circle {
                        fill_color: colors::OVERLAY,
                        shape: dequeue_circle::background_at(side, row, 2),
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

fn arsenal_display(args: ScoreboardRenderArgs) -> Vec<Component> {
    let ScoreboardRenderArgs { player, side, .. } = args;

    let row_offset = helpers::height_in_rows(&player.queue.pool, 3) + 1;

    let pill = Pill {
        position: CirclePosition {
            from: side,
            column: 0,
            row: row_offset,
        },
        width_in_columns: 3,
        height_in_rows: helpers::height_in_rows(&player.arsenal, 3),
        enabled: false,
    };

    let arsenal_items = player
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
                    shape: dequeue_circle::background_at(side, row, column),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::from(arsenal_item),
                    alpha: colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0,
                    shape: dequeue_circle::foreground_at(side, row, column),
                    on_click: None,
                },
                Component::Circle {
                    fill_color: colors::OVERLAY,
                    shape: dequeue_circle::background_at(side, row, column),
                    on_click: None,
                },
            ]
        });

    pill.render().into_iter().chain(arsenal_items).collect()
}

fn arrows(args: ScoreboardRenderArgs) -> Vec<Component> {
    let ScoreboardRenderArgs { player, side, .. } = args;

    let pool_height_in_rows = helpers::height_in_rows(&player.queue.pool, 3);
    let entrance_and_exit_to_pool = if pool_height_in_rows == 0 {
        vec![]
    } else {
        vec![
            arrow::up_arrow_above(side, pool_height_in_rows, 0),
            arrow::down_arrow_above(side, pool_height_in_rows, 2),
        ]
    };
    let arsenal_to_entrance_and_exit = vec![
        arrow::up_arrow_above(side, pool_height_in_rows + 1, 0),
        arrow::down_arrow_above(side, pool_height_in_rows + 1, 2),
    ];

    entrance_and_exit_to_pool
        .into_iter()
        .chain(arsenal_to_entrance_and_exit)
        .collect()
}

struct ScoreboardRenderArgs<'a> {
    pub player: &'a DequeueingPlayer,
    pub side: Side,
    pub dequeues: &'a Vec<DequeueChoice>,
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
