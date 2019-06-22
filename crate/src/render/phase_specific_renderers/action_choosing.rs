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
        rect_button, rect_focus, Translate,
    },
    side::Side,
};

use nzscq::{
    choices::{Action as NzscAction, ArsenalItem, Booster, DequeueChoice},
    scoreboard::{ActionlessPlayer, DequeueingPlayer},
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
                    self.human_dequeued_items(),
                ),
                dequeueing_scoreboard(self.dequeueing_computer_args()),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                entering_dequeue_choice(Side::Left, self.human_dequeued_items(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn human_dequeued_items(&self) -> DequeuedItems {
        let drainee = if let DequeueChoice::DrainAndExit(drainee) = self.previous_outcome[HUMAN] {
            Some(drainee)
        } else {
            None
        };
        let exiting_item = self.previous_scoreboard[HUMAN].queue.exit;

        DequeuedItems {
            drainee,
            exiting_item,
        }
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
                    self.human_dequeued_items(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeued_items(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                stationary_dequeue_choice(Side::Left, self.human_dequeued_items()),
                entering_dequeue_choice(Side::Right, self.computer_dequeued_items(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn computer_dequeued_items(&self) -> DequeuedItems {
        let drainee = if let DequeueChoice::DrainAndExit(drainee) = self.previous_outcome[COMPUTER]
        {
            Some(drainee)
        } else {
            None
        };
        let exiting_item = self.previous_scoreboard[COMPUTER].queue.exit;

        DequeuedItems {
            drainee,
            exiting_item,
        }
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
                    self.human_dequeued_items(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeued_items(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                stationary_dequeue_choice(Side::Left, self.human_dequeued_items()),
                stationary_dequeue_choice(Side::Right, self.computer_dequeued_items()),
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
                    self.human_dequeued_items(),
                ),
                dequeueing_scoreboard_without_dequeued_items(
                    self.dequeueing_computer_args(),
                    self.computer_dequeued_items(),
                ),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                exiting_dequeue_choice(Side::Left, self.human_dequeued_items(), &lerper),
                exiting_dequeue_choice(Side::Right, self.computer_dequeued_items(), &lerper),
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
    items: DequeuedItems,
) -> Vec<Component> {
    let DequeuedItems { drainee, .. } = items;

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

fn entering_dequeue_choice(side: Side, items: DequeuedItems, lerper: &Lerper) -> Vec<Component> {
    vec![]
}

fn stationary_dequeue_choice(side: Side, items: DequeuedItems) -> Vec<Component> {
    vec![]
}

fn exiting_dequeue_choice(side: Side, items: DequeuedItems, lerper: &Lerper) -> Vec<Component> {
    vec![]
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
                let row = i / 3;
                let column = i % 3;

                if drain_and_exit_enabled {
                    vec![
                        Component::Circle {
                            fill_color: colors::arsenal_item_color(arsenal_item),
                            shape: dequeue_circle::background_at(side, row, column),
                            on_click: None,
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
        } else if just_exit_enabled {
            exit.map(|exiting_item| {
                vec![
                    Component::Circle {
                        fill_color: colors::arsenal_item_color(exiting_item),
                        shape: dequeue_circle::background_at(side, row, 2),
                        on_click: None,
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
                action_choosing_arsenal_item_display(
                    arsenal_item,
                    false,
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
            action_choosing_arsenal_item_display(
                entering_item,
                false,
                CirclePosition {
                    from: side,
                    column: 0,
                    row,
                },
            )
        }),
        exit.map(|exiting_item| {
            action_choosing_arsenal_item_display(
                exiting_item,
                false,
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

            vec![
                Component::Circle {
                    fill_color: colors::arsenal_item_color(arsenal_item),
                    shape: dequeue_circle::background_at(side, row, column),
                    on_click: side.if_left(()).and(opt_move).map(|m| Action::ChooseAction(NzscAction::Move(m))),
                },
                Component::Image {
                    image_type: ImageType::from(arsenal_item),
                    alpha: 1.0,
                    shape: dequeue_circle::foreground_at(side, row, column),
                    on_click: None,
                },
            ]
        });

    pill.render().into_iter().chain(arsenal_items).collect()
}

fn action_choosing_arsenal_item_display(
    item: ArsenalItem,
    enabled: bool,
    position: CirclePosition,
) -> Vec<Component> {
    let CirclePosition { from, column, row } = position;
    let side = from;
    let fill_color = if enabled {

        colors::arsenal_item_color(item)
    } else {
        colors::arsenal_item_color(item).with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA)
    };
    let image_alpha = if enabled {
        1.0
    } else {
        colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA as f64 / 255.0
    };

    vec![
        vec![
            Component::Circle {
                fill_color: fill_color,
                shape: dequeue_circle::background_at(side, row, column),
                on_click: None,
            },
            Component::Image {
                image_type: ImageType::from(item),
                alpha: image_alpha,
                shape: dequeue_circle::foreground_at(side, row, column),
                on_click: None,
            },
        ],
        if enabled {
            vec![]
        } else if item == ArsenalItem::Mirror {
            vec![]
        } else {
            vec![Component::Circle {
                fill_color: colors::OVERLAY,
                shape: dequeue_circle::background_at(side, row, column),
                on_click: None,
            }]
        },
    ]
    .into_iter()
    .flatten()
    .collect()
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

struct DequeuedItems {
    pub drainee: Option<ArsenalItem>,
    pub exiting_item: Option<ArsenalItem>,
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;

// TODO disallow human to choose a computer's move