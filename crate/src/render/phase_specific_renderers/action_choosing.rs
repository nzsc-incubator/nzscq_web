use crate::{
    click::Action,
    colors, helpers,
    paint::{Component, ImageType},
    render::{
        arrow, arsenal_item_display,
        health_display::ConstantHealthDisplay,
        lerp::{LerpableComponent, Lerper},
        pill::Pill,
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{
        dequeue_circle::{self, CirclePosition},
        dequeue_foci,
    },
    side::Side,
};

use nzscq::{
    choices::{Action as NzscAction, ArsenalItem, DequeueChoice},
    scoreboard::{ActionlessPlayer, DequeueingPlayer, Queue},
};

pub struct ActionChoosingPhaseRenderer<'a> {
    pub completion_factor: f64,
    pub previous_scoreboard: &'a [DequeueingPlayer; 2],
    pub previously_available_dequeues: &'a [Vec<DequeueChoice>; 2],
    pub previous_outcome: &'a [DequeueChoice; 2],
    pub scoreboard: &'a [ActionlessPlayer; 2],
    pub available_actions: &'a [Vec<NzscAction>; 2],
}

impl<'a> ActionChoosingPhaseRenderer<'a> {
    pub fn render(self) -> Vec<Component> {
        let human_entrance = self.human_entrance();
        let computer_entrance = self.computer_entrance();
        let pause = self.pause();
        let exit = self.exit();
        let actions = self.actions();

        Switch5(
            (0.00..0.15, human_entrance),
            (0.15..0.30, computer_entrance),
            (0.30..0.85, pause),
            (0.85..1.00, exit),
            (1.00..=1.00, actions),
        )
        .case(self.completion_factor)
        .expect("should have legal completion range")
    }

    fn human_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_human_args(),
                    self.human_dequeue_displacements(),
                ),
                dequeueing_scoreboard(self.dequeueing_computer_args()),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                entering_dequeue_choice(Side::Left, self.human_dequeue_displacements(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn human_dequeue_displacements(&self) -> DequeueDisplacements {
        let previous_human = &self.previous_scoreboard[HUMAN];
        let current_human = &self.scoreboard[HUMAN];
        let drainee = if let DequeueChoice::DrainAndExit(drainee) = self.previous_outcome[HUMAN] {
            Some(ArsenalItemDisplacement {
                item: drainee,
                start: position_of(drainee, previous_human, Side::Left)
                    .expect("drainee should have previous position"),
                end: position_of(drainee, current_human, Side::Left)
                    .expect("drainee should have current position"),
            })
        } else {
            None
        };
        let exiter = self.previous_scoreboard[HUMAN]
            .queue
            .exit
            .filter(|_| self.previous_outcome[HUMAN] != DequeueChoice::Decline)
            .map(|exiter| ArsenalItemDisplacement {
                item: exiter,
                start: position_of(exiter, previous_human, Side::Left)
                    .expect("exiter should have previous position"),
                end: position_of(exiter, current_human, Side::Left)
                    .expect("exiter should have current position"),
            });

        DequeueDisplacements { drainee, exiter }
    }

    fn computer_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_human_args(),
                    self.human_dequeue_displacements(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeue_displacements(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                stationary_dequeue_choice(Side::Left, self.human_dequeue_displacements()),
                entering_dequeue_choice(
                    Side::Right,
                    self.computer_dequeue_displacements(),
                    &lerper,
                ),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn computer_dequeue_displacements(&self) -> DequeueDisplacements {
        let previous_computer = &self.previous_scoreboard[COMPUTER];
        let current_computer = &self.scoreboard[COMPUTER];
        let drainee = if let DequeueChoice::DrainAndExit(drainee) = self.previous_outcome[COMPUTER]
        {
            Some(ArsenalItemDisplacement {
                item: drainee,
                start: position_of(drainee, previous_computer, Side::Right)
                    .expect("drainee should have previous position"),
                end: position_of(drainee, current_computer, Side::Right)
                    .expect("drainee should have current position"),
            })
        } else {
            None
        };
        let exiter = self.previous_scoreboard[COMPUTER]
            .queue
            .exit
            .filter(|_| self.previous_outcome[COMPUTER] != DequeueChoice::Decline)
            .map(|exiter| ArsenalItemDisplacement {
                item: exiter,
                start: position_of(exiter, previous_computer, Side::Right)
                    .expect("exiter should have previous position"),
                end: position_of(exiter, current_computer, Side::Right)
                    .expect("exiter should have current position"),
            });

        DequeueDisplacements { drainee, exiter }
    }

    fn pause(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |_lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_human_args(),
                    self.human_dequeue_displacements(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeue_displacements(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                stationary_dequeue_choice(Side::Left, self.human_dequeue_displacements()),
                stationary_dequeue_choice(Side::Right, self.computer_dequeue_displacements()),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn exit(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_human_args(),
                    self.human_dequeue_displacements(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeue_displacements(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                exiting_dequeue_choice(Side::Left, self.human_dequeue_displacements(), &lerper),
                exiting_dequeue_choice(Side::Right, self.computer_dequeue_displacements(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn actions(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |_lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.health_display(),
                action_choosing_scoreboard(self.actionless_human_args()),
                action_choosing_scoreboard(self.actionless_computer_args()),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn dequeueing_human_args(&self) -> DequeueingRenderArgs {
        DequeueingRenderArgs {
            player: &self.previous_scoreboard[HUMAN],
            side: Side::Left,
            dequeues: &self.previously_available_dequeues[HUMAN],
        }
    }

    fn dequeueing_computer_args(&self) -> DequeueingRenderArgs {
        DequeueingRenderArgs {
            player: &self.previous_scoreboard[COMPUTER],
            side: Side::Right,
            dequeues: &self.previously_available_dequeues[COMPUTER],
        }
    }

    fn actionless_human_args(&self) -> ActionChoosingRenderArgs {
        ActionChoosingRenderArgs {
            player: &self.scoreboard[HUMAN],
            side: Side::Left,
            actions: &self.available_actions[HUMAN],
        }
    }

    fn actionless_computer_args(&self) -> ActionChoosingRenderArgs {
        ActionChoosingRenderArgs {
            player: &self.scoreboard[COMPUTER],
            side: Side::Right,
            actions: &self.available_actions[COMPUTER],
        }
    }

    fn health_display(&self) -> Vec<Component> {
        ConstantHealthDisplay {
            human_health: helpers::opponent_points_to_own_health(self.computer_points()),
            computer_health: helpers::opponent_points_to_own_health(self.human_points()),
        }
        .render()
    }

    fn human_points(&self) -> u8 {
        self.scoreboard[HUMAN].points
    }

    fn computer_points(&self) -> u8 {
        self.scoreboard[COMPUTER].points
    }
}

fn dequeueing_scoreboard_without_dequeued_items(
    args: DequeueingRenderArgs,
    displacements: DequeueDisplacements,
) -> Vec<Component> {
    let drainee = displacements.drainee.map(|displacement| displacement.item);

    vec![
        dequeueing_pool_display_without_drainee(&args, drainee),
        dequeueing_entrance_and_decline_display(&args),
        dequeueing_arsenal_display(&args),
        arrows(&args),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn dequeueing_scoreboard(args: DequeueingRenderArgs) -> Vec<Component> {
    vec![
        dequeueing_pool_display(&args),
        dequeueing_entrance_decline_and_exit_display(&args),
        dequeueing_arsenal_display(&args),
        arrows(&args),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn entering_dequeue_choice(
    side: Side,
    displacements: DequeueDisplacements,
    lerper: &Lerper,
) -> Vec<Component> {
    vec![
        displacements.drainee.map(|displacement| {
            let drainee = displacement.item;
            let position = displacement.start;

            vec![
                LerpableComponent::Circle {
                    start_color: colors::arsenal_item_color(drainee),
                    end_color: colors::arsenal_item_color(drainee),
                    start_shape: dequeue_circle::background_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    end_shape: dequeue_foci::top_background(side),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::from(drainee),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: dequeue_circle::foreground_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    end_shape: dequeue_foci::top_foreground(side),
                    on_click: None,
                },
            ]
        }),
        displacements.exiter.map(|displacement| {
            let exiter = displacement.item;
            let position = displacement.start;

            vec![
                LerpableComponent::Circle {
                    start_color: colors::arsenal_item_color(exiter),
                    end_color: colors::arsenal_item_color(exiter),
                    start_shape: dequeue_circle::background_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    end_shape: dequeue_foci::bottom_background(side),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::from(exiter),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: dequeue_circle::foreground_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    end_shape: dequeue_foci::bottom_foreground(side),
                    on_click: None,
                },
            ]
        }),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .map(|lerpable: LerpableComponent| lerper.lerp1(lerpable))
    .collect()
}

fn stationary_dequeue_choice(side: Side, displacements: DequeueDisplacements) -> Vec<Component> {
    vec![
        displacements.drainee.map(|displacement| {
            let drainee = displacement.item;

            vec![
                Component::Circle {
                    fill_color: colors::arsenal_item_color(drainee),
                    shape: dequeue_foci::top_background(side),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::from(drainee),
                    alpha: 1.0,
                    shape: dequeue_foci::top_foreground(side),
                    on_click: None,
                },
            ]
        }),
        displacements.exiter.map(|displacement| {
            let exiter = displacement.item;

            vec![
                Component::Circle {
                    fill_color: colors::arsenal_item_color(exiter),
                    shape: dequeue_foci::bottom_background(side),
                    on_click: None,
                },
                Component::Image {
                    image_type: ImageType::from(exiter),
                    alpha: 1.0,
                    shape: dequeue_foci::bottom_foreground(side),
                    on_click: None,
                },
            ]
        }),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .collect()
}

fn exiting_dequeue_choice(
    side: Side,
    displacements: DequeueDisplacements,
    lerper: &Lerper,
) -> Vec<Component> {
    vec![
        displacements.drainee.map(|displacement| {
            let drainee = displacement.item;
            let position = displacement.end;

            vec![
                LerpableComponent::Circle {
                    start_color: colors::arsenal_item_color(drainee),
                    end_color: colors::arsenal_item_color(drainee),
                    start_shape: dequeue_foci::top_background(side),
                    end_shape: dequeue_circle::background_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::from(drainee),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: dequeue_foci::top_foreground(side),
                    end_shape: dequeue_circle::foreground_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    on_click: None,
                },
            ]
        }),
        displacements.exiter.map(|displacement| {
            let exiter = displacement.item;
            let position = displacement.end;

            vec![
                LerpableComponent::Circle {
                    start_color: colors::arsenal_item_color(exiter),
                    end_color: colors::arsenal_item_color(exiter),
                    start_shape: dequeue_foci::bottom_background(side),
                    end_shape: dequeue_circle::background_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    on_click: None,
                },
                LerpableComponent::Image {
                    image_type: ImageType::from(exiter),
                    start_alpha: 1.0,
                    end_alpha: 1.0,
                    start_shape: dequeue_foci::bottom_foreground(side),
                    end_shape: dequeue_circle::foreground_at(
                        position.from,
                        position.row,
                        position.column,
                    ),
                    on_click: None,
                },
            ]
        }),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .map(|lerpable: LerpableComponent| lerper.lerp1(lerpable))
    .collect()
}

fn action_choosing_scoreboard(args: ActionChoosingRenderArgs) -> Vec<Component> {
    vec![
        action_choosing_pool_display(&args),
        action_choosing_entrance_and_exit_display(&args),
        action_choosing_arsenal_display(&args),
        arrows(&args),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn dequeueing_pool_display_without_drainee(
    args: &DequeueingRenderArgs,
    drainee: Option<ArsenalItem>,
) -> Vec<Component> {
    let DequeueingRenderArgs {
        player,
        side,
        dequeues,
    } = args;
    let side = *side;

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
            if Some(arsenal_item) == drainee {
                vec![]
            } else {
                arsenal_item_display(
                    arsenal_item,
                    drain_and_exit_enabled,
                    None,
                    CirclePosition {
                        from: side,
                        column: i % 3,
                        row: i / 3,
                    },
                )
            }
        });

    pill.render().into_iter().chain(pool).collect()
}

fn dequeueing_pool_display(args: &DequeueingRenderArgs) -> Vec<Component> {
    dequeueing_pool_display_without_drainee(args, None)
}

fn dequeueing_entrance_and_decline_display(args: &DequeueingRenderArgs) -> Vec<Component> {
    dequeueing_entrance_decline_and_exit_display_with_hidable_exit(args, true)
}

fn dequeueing_entrance_decline_and_exit_display(args: &DequeueingRenderArgs) -> Vec<Component> {
    dequeueing_entrance_decline_and_exit_display_with_hidable_exit(args, false)
}

fn dequeueing_entrance_decline_and_exit_display_with_hidable_exit(
    args: &DequeueingRenderArgs,
    hide_exit: bool,
) -> Vec<Component> {
    let DequeueingRenderArgs {
        player,
        side,
        dequeues,
    } = args;
    let side = *side;

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
            arsenal_item_display(
                entering_item,
                false,
                None,
                CirclePosition {
                    from: side,
                    column: 0,
                    row,
                },
            )
        }),
        Some(vec![
            Component::Circle {
                fill_color: colors::DECLINE_DEQUEUE_COLOR,
                shape: dequeue_circle::background_at(side, row, 1),
                on_click: None,
            },
            Component::Image {
                image_type: ImageType::DeclineDequeue,
                alpha: 1.0,
                shape: dequeue_circle::foreground_at(side, row, 1),
                on_click: None,
            },
        ]),
        if hide_exit {
            None
        } else {
            exit.map(|exiter| {
                arsenal_item_display(
                    exiter,
                    just_exit_enabled,
                    None,
                    CirclePosition {
                        from: side,
                        column: 2,
                        row,
                    },
                )
            })
        },
    ]
    .into_iter()
    .flatten()
    .flatten()
    .collect()
}

fn dequeueing_arsenal_display(args: &DequeueingRenderArgs) -> Vec<Component> {
    let DequeueingRenderArgs { player, side, .. } = args;
    let side = *side;

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

            arsenal_item_display(
                arsenal_item,
                false,
                None,
                CirclePosition {
                    from: side,
                    column,
                    row,
                },
            )
        });

    pill.render().into_iter().chain(arsenal_items).collect()
}

fn arrows<T: ArrowRenderArgs>(args: &T) -> Vec<Component> {
    let side = args.side();
    let pool = args.pool();

    let pool_height_in_rows = helpers::height_in_rows(pool, 3);
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

fn action_choosing_pool_display(args: &ActionChoosingRenderArgs) -> Vec<Component> {
    let ActionChoosingRenderArgs {
        player,
        side,
        actions,
    } = args;
    let side = *side;

    let mirror_enabled = actions.iter().any(|action| {
        if let NzscAction::Mirror(_) = action {
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
        enabled: mirror_enabled,
    };

    let pool = player
        .queue
        .pool
        .iter()
        .enumerate()
        .flat_map(|(i, &arsenal_item)| {
            let row = i / 3;
            let column = i % 3;

            if mirror_enabled {
                let move_ = match arsenal_item {
                    ArsenalItem::Move(move_) => move_,
                    ArsenalItem::Mirror => panic!(
                        "The pool should not contain a mirror if mirroring is a legal action"
                    ),
                };

                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(arsenal_item),
                        shape: dequeue_circle::background_at(side, row, column),
                        on_click: side.if_left(Action::ChooseAction(NzscAction::Mirror(move_))),
                    },
                    Component::Image {
                        image_type: ImageType::Mirror,
                        alpha: 1.0,
                        shape: dequeue_circle::foreground_at(side, row, column),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::from(arsenal_item),
                        alpha: 1.0,
                        // TODO shrink the move image
                        shape: dequeue_circle::foreground_at(side, row, column),
                        on_click: None,
                    },
                ]
            } else {
                arsenal_item_display(
                    arsenal_item,
                    false,
                    None,
                    CirclePosition {
                        from: side,
                        column,
                        row,
                    },
                )
            }
        });

    pill.render().into_iter().chain(pool).collect()
}

fn action_choosing_entrance_and_exit_display(args: &ActionChoosingRenderArgs) -> Vec<Component> {
    let ActionChoosingRenderArgs { player, side, .. } = args;
    let side = *side;

    let entrance = player.queue.entrance;
    let exit = player.queue.exit;
    let pool_height_in_rows = helpers::height_in_rows(&player.queue.pool, 3);
    let row = pool_height_in_rows;

    let pill = Pill {
        position: CirclePosition {
            from: side,
            column: 0,
            row: pool_height_in_rows,
        },
        width_in_columns: 3,
        height_in_rows: 1,
        enabled: false,
    };

    vec![
        Some(pill.render()),
        entrance.map(|entering_item| {
            arsenal_item_display(
                entering_item,
                false,
                None,
                CirclePosition {
                    from: side,
                    column: 0,
                    row,
                },
            )
        }),
        exit.map(|exiting_item| {
            arsenal_item_display(
                exiting_item,
                false,
                None,
                CirclePosition {
                    from: side,
                    column: 2,
                    row,
                },
            )
        }),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .collect()
}

fn action_choosing_arsenal_display(args: &ActionChoosingRenderArgs) -> Vec<Component> {
    let ActionChoosingRenderArgs { player, side, .. } = args;
    let side = *side;

    let row_offset = helpers::height_in_rows(&player.queue.pool, 3) + 1;

    let pill = Pill {
        position: CirclePosition {
            from: side,
            column: 0,
            row: row_offset,
        },
        width_in_columns: 3,
        height_in_rows: helpers::height_in_rows(&player.arsenal, 3),
        enabled: true,
    };

    let arsenal_items = player
        .arsenal
        .iter()
        .enumerate()
        .flat_map(|(i, &arsenal_item)| {
            let row = i / 3;
            let column = i % 3;
            let row = row + row_offset;
            let opt_move = if let ArsenalItem::Move(m) = arsenal_item {
                Some(m)
            } else {
                None
            };

            arsenal_item_display(
                arsenal_item,
                true,
                side.if_left(())
                    .and(opt_move)
                    .map(|m| Action::ChooseAction(NzscAction::Move(m))),
                CirclePosition {
                    from: side,
                    column,
                    row,
                },
            )
        });

    pill.render().into_iter().chain(arsenal_items).collect()
}

fn position_of(item: ArsenalItem, player: &QueueArsenal, side: Side) -> Option<CirclePosition> {
    let index = player
        .queue()
        .pool
        .iter()
        .position(|&pool_item| pool_item == item);
    if let Some(index) = index {
        Some(CirclePosition {
            from: side,
            column: index % 3,
            row: index / 3,
        })
    } else {
        position_in_mouth_or_arsenal_of(item, player, side)
    }
}

fn position_in_mouth_or_arsenal_of(
    item: ArsenalItem,
    player: &QueueArsenal,
    side: Side,
) -> Option<CirclePosition> {
    let pool_height = helpers::height_in_rows(&player.queue().pool, 3);
    if Some(item) == player.queue().entrance {
        Some(CirclePosition {
            from: side,
            column: 0,
            row: pool_height,
        })
    } else if Some(item) == player.queue().exit {
        Some(CirclePosition {
            from: side,
            column: 2,
            row: pool_height,
        })
    } else {
        position_in_arsenal_of(item, player, side)
    }
}

fn position_in_arsenal_of(
    item: ArsenalItem,
    player: &QueueArsenal,
    side: Side,
) -> Option<CirclePosition> {
    let index = player
        .arsenal()
        .iter()
        .position(|&arsenal_item| arsenal_item == item);
    index.map(|index| {
        let pool_height = helpers::height_in_rows(&player.queue().pool, 3);
        let mouth_height = 1;
        let row_offset = pool_height + mouth_height;

        CirclePosition {
            from: side,
            column: index % 3,
            row: row_offset + index / 3,
        }
    })
}

trait QueueArsenal {
    fn queue(&self) -> &Queue;
    fn arsenal(&self) -> &Vec<ArsenalItem>;
}

impl QueueArsenal for DequeueingPlayer {
    fn queue(&self) -> &Queue {
        &self.queue
    }

    fn arsenal(&self) -> &Vec<ArsenalItem> {
        &self.arsenal
    }
}

impl QueueArsenal for ActionlessPlayer {
    fn queue(&self) -> &Queue {
        &self.queue
    }

    fn arsenal(&self) -> &Vec<ArsenalItem> {
        &self.arsenal
    }
}

struct DequeueingRenderArgs<'a> {
    pub player: &'a DequeueingPlayer,
    pub side: Side,
    pub dequeues: &'a Vec<DequeueChoice>,
}

struct ActionChoosingRenderArgs<'a> {
    pub player: &'a ActionlessPlayer,
    pub side: Side,
    pub actions: &'a Vec<NzscAction>,
}

trait ArrowRenderArgs {
    fn side(&self) -> Side;
    fn pool(&self) -> &Vec<ArsenalItem>;
}

impl<'a> ArrowRenderArgs for DequeueingRenderArgs<'a> {
    fn side(&self) -> Side {
        self.side
    }
    fn pool(&self) -> &Vec<ArsenalItem> {
        &self.player.queue.pool
    }
}

impl<'a> ArrowRenderArgs for ActionChoosingRenderArgs<'a> {
    fn side(&self) -> Side {
        self.side
    }
    fn pool(&self) -> &Vec<ArsenalItem> {
        &self.player.queue.pool
    }
}

struct DequeueDisplacements {
    pub drainee: Option<ArsenalItemDisplacement>,
    pub exiter: Option<ArsenalItemDisplacement>,
}

struct ArsenalItemDisplacement {
    pub item: ArsenalItem,
    pub start: CirclePosition,
    pub end: CirclePosition,
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
