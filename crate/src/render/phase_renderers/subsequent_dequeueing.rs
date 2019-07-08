use crate::{
    click::Action,
    colors, helpers,
    paint::{Component, ImageType},
    phase::{ChooseSubsequentDequeuePhase, MoveInspectorState},
    render::{
        arrow, arsenal_item_display,
        health_display::{ConstantHealthDisplay, FadingHealthDisplay},
        lerp::{LerpableComponent, Lerper},
        move_inspector_buttons::RenderButton,
        pill::Pill,
        switch::{Switch, Switch5},
        Render,
    },
    shapes::{
        action_focus,
        dequeue_circle::{self, CirclePosition},
    },
    side::Side,
};

use nzscq::{
    choices::{Action as NzscAction, ArsenalItem, DequeueChoice},
    outcomes::ActionPointsDestroyed,
    scoreboard::{ActionlessPlayer, DequeueingPlayer, Queue},
};

pub struct SubsequentDequeueingPhaseRenderer<'a> {
    previous_scoreboard: &'a [ActionlessPlayer; 2],
    previously_available_actions: &'a [Vec<NzscAction>; 2],
    previous_outcome: &'a [ActionPointsDestroyed; 2],
    scoreboard: &'a [DequeueingPlayer; 2],
    available_dequeues: &'a [Vec<DequeueChoice>; 2],
    inspector_state: MoveInspectorState,
}

impl<'a> SubsequentDequeueingPhaseRenderer<'a> {
    pub fn new(phase: &'a ChooseSubsequentDequeuePhase) -> SubsequentDequeueingPhaseRenderer<'a> {
        SubsequentDequeueingPhaseRenderer {
            previous_scoreboard: &phase.previous_scoreboard,
            previously_available_actions: &phase.previously_available_actions,
            previous_outcome: &phase.previous_outcome,
            scoreboard: &phase.scoreboard,
            available_dequeues: &phase.available_dequeues,
            inspector_state: phase.inspector_state,
        }
    }

    fn human_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.previous_health_displays(),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_human_args(),
                    self.human_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                action_choosing_scoreboard(self.action_choosing_computer_args()),
                self.inspector_state.render_button(false),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                entering_action(Side::Left, self.human_action_displacement(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn human_action_displacement(&self) -> Option<ActionVisit> {
        let action = self.previous_outcome[HUMAN].0;
        let previous_human = &self.previous_scoreboard[HUMAN];
        let current_human = &self.scoreboard[HUMAN];

        let arsenal_item: Option<ArsenalItem> = action.into();

        arsenal_item.map(|item| ActionVisit {
            action,
            start: position_of(item, previous_human, Side::Left)
                .expect("used item should have previous position"),
            end: position_of(item, current_human, Side::Left),
        })
    }

    fn computer_entrance(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.previous_health_displays(),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_human_args(),
                    self.human_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_computer_args(),
                    self.computer_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                self.inspector_state.render_button(false),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                stationary_action(Side::Left, self.human_action_displacement()),
                entering_action(Side::Right, self.computer_action_displacement(), &lerper),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn computer_action_displacement(&self) -> Option<ActionVisit> {
        let action = self.previous_outcome[COMPUTER].0;
        let previous_computer = &self.previous_scoreboard[COMPUTER];
        let current_computer = &self.scoreboard[COMPUTER];

        let arsenal_item: Option<ArsenalItem> = action.into();

        arsenal_item.map(|item| ActionVisit {
            action,
            start: position_of(item, previous_computer, Side::Right)
                .expect("used item should have previous position"),
            end: position_of(item, current_computer, Side::Right),
        })
    }

    fn fade(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_human_args(),
                    self.human_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_computer_args(),
                    self.computer_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                self.inspector_state.render_button(false),
                self.fade_case_non_fading_health_displays(),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                self.fade_case_fading_health_displays(&lerper),
                if self.did_computer_get_point() {
                    fading_action(Side::Left, self.human_action_displacement(), &lerper)
                } else {
                    stationary_action(Side::Left, self.human_action_displacement())
                },
                if self.did_human_get_point() {
                    fading_action(Side::Right, self.computer_action_displacement(), &lerper)
                } else {
                    stationary_action(Side::Right, self.computer_action_displacement())
                },
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
                self.current_health_displays(),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_human_args(),
                    self.human_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                action_choosing_scoreboard_without_used_item(
                    self.action_choosing_computer_args(),
                    self.computer_action_displacement()
                        .and_then(|displacement| displacement.action.into()),
                ),
                self.inspector_state.render_button(false),
                vec![Component::Background {
                    color: colors::OVERLAY,
                }],
                if self.did_computer_get_point() {
                    vec![]
                } else {
                    exiting_action(Side::Left, self.human_action_displacement(), &lerper)
                },
                if self.did_human_get_point() {
                    vec![]
                } else {
                    exiting_action(Side::Right, self.computer_action_displacement(), &lerper)
                },
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn dequeues(&'a self) -> impl 'a + FnOnce(Lerper) -> Vec<Component> {
        move |_lerper| {
            vec![
                vec![Component::Background {
                    color: colors::BACKGROUND,
                }],
                self.current_health_displays(),
                dequeueing_scoreboard(self.dequeueing_human_args()),
                dequeueing_scoreboard(self.dequeueing_computer_args()),
                self.inspector_state.render_button(true),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    fn action_choosing_human_args(&self) -> ActionChoosingRenderArgs {
        ActionChoosingRenderArgs {
            player: &self.previous_scoreboard[HUMAN],
            side: Side::Left,
            actions: &self.previously_available_actions[HUMAN],
        }
    }

    fn action_choosing_computer_args(&self) -> ActionChoosingRenderArgs {
        ActionChoosingRenderArgs {
            player: &self.previous_scoreboard[COMPUTER],
            side: Side::Right,
            actions: &self.previously_available_actions[COMPUTER],
        }
    }

    fn dequeueing_human_args(&self) -> DequeueingRenderArgs {
        DequeueingRenderArgs {
            player: &self.scoreboard[HUMAN],
            side: Side::Left,
            dequeues: &self.available_dequeues[HUMAN],
        }
    }

    fn dequeueing_computer_args(&self) -> DequeueingRenderArgs {
        DequeueingRenderArgs {
            player: &self.scoreboard[COMPUTER],
            side: Side::Right,
            dequeues: &self.available_dequeues[COMPUTER],
        }
    }

    fn previous_health_displays(&self) -> Vec<Component> {
        let human_display = ConstantHealthDisplay {
            side: Side::Left,
            health: self.previous_human_health(),
        };
        let computer_display = ConstantHealthDisplay {
            side: Side::Right,
            health: self.previous_computer_health(),
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
                starting_health: self.previous_human_health(),
            })
        } else {
            vec![]
        };
        let computer_components = if self.did_human_get_point() {
            lerper.lerp1(FadingHealthDisplay {
                side: Side::Right,
                starting_health: self.previous_computer_health(),
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
                health: self.previous_human_health(),
            }
            .render(())
        };
        let computer_components = if self.did_human_get_point() {
            vec![]
        } else {
            ConstantHealthDisplay {
                side: Side::Right,
                health: self.previous_computer_health(),
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
            health: self.previous_human_health()
                - if self.did_computer_get_point() { 1 } else { 0 },
        };
        let computer_display = ConstantHealthDisplay {
            side: Side::Right,
            health: self.previous_computer_health()
                - if self.did_human_get_point() { 1 } else { 0 },
        };

        vec![human_display, computer_display]
            .into_iter()
            .map(|display| display.render(()))
            .flatten()
            .collect()
    }

    fn previous_human_health(&self) -> u8 {
        let opponent_points = self.previous_scoreboard[COMPUTER].points;

        helpers::opponent_points_to_own_health(opponent_points)
    }

    fn previous_computer_health(&self) -> u8 {
        let opponent_points = self.previous_scoreboard[HUMAN].points;

        helpers::opponent_points_to_own_health(opponent_points)
    }

    fn did_human_get_point(&self) -> bool {
        self.previous_outcome[HUMAN].1 > 0
    }

    fn did_computer_get_point(&self) -> bool {
        self.previous_outcome[COMPUTER].1 > 0
    }
}

impl<'a> Render<f64> for SubsequentDequeueingPhaseRenderer<'a> {
    fn render(&self, completion_factor: f64) -> Vec<Component> {
        let human_entrance = self.human_entrance();
        let computer_entrance = self.computer_entrance();
        let fade = self.fade();
        let exit = self.exit();
        let dequeues = self.dequeues();

        Switch5(
            (0.00..0.15, human_entrance),
            (0.15..0.30, computer_entrance),
            (0.30..0.85, fade),
            (0.85..1.00, exit),
            (1.00..=1.00, dequeues),
        )
        .case(completion_factor)
        .expect("should have legal completion range")
    }
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

fn entering_action(side: Side, visit: Option<ActionVisit>, lerper: &Lerper) -> Vec<Component> {
    visit
        .map(|visit| {
            let ActionVisit { action, start, .. } = visit;

            match action {
                NzscAction::Concede => panic!("optional visit should be None if Action is Concede"),
                NzscAction::Mirror(move_) => {
                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_),
                            start_shape: dequeue_circle::background_at(
                                start.side,
                                start.row,
                                start.column,
                            ),
                            end_shape: action_focus::background(side),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Mirror,
                            start_alpha: 1.0,
                            end_alpha: 1.0,
                            start_shape: dequeue_circle::foreground_at(
                                start.side,
                                start.row,
                                start.column,
                            ),
                            end_shape: action_focus::foreground(side),
                            on_click: None,
                        },
                        // TODO Shrink move image
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 1.0,
                            start_shape: dequeue_circle::foreground_at(
                                start.side,
                                start.row,
                                start.column,
                            ),
                            end_shape: action_focus::foreground(side),
                            on_click: None,
                        },
                    ]
                }
                NzscAction::Move(move_) => vec![
                    LerpableComponent::Circle {
                        start_color: colors::move_color(move_),
                        end_color: colors::move_color(move_),
                        start_shape: dequeue_circle::background_at(
                            start.side,
                            start.row,
                            start.column,
                        ),
                        end_shape: action_focus::background(side),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Move(move_),
                        start_alpha: 1.0,
                        end_alpha: 1.0,
                        start_shape: dequeue_circle::foreground_at(
                            start.side,
                            start.row,
                            start.column,
                        ),
                        end_shape: action_focus::foreground(side),
                        on_click: None,
                    },
                ],
            }
        })
        .into_iter()
        .flatten()
        .map(|lerpable: LerpableComponent| lerper.lerp1(lerpable))
        .collect()
}

fn stationary_action(side: Side, visit: Option<ActionVisit>) -> Vec<Component> {
    visit
        .map(|visit| {
            let ActionVisit { action, .. } = visit;

            match action {
                NzscAction::Concede => panic!("optional visit should be None if Action is Concede"),
                NzscAction::Mirror(move_) => {
                    vec![
                        Component::Circle {
                            fill_color: colors::move_color(move_),
                            shape: action_focus::background(side),
                            on_click: None,
                        },
                        Component::Image {
                            image_type: ImageType::Mirror,
                            alpha: 1.0,
                            shape: action_focus::foreground(side),
                            on_click: None,
                        },
                        // TODO Shrink move image
                        Component::Image {
                            image_type: ImageType::Move(move_),
                            alpha: 1.0,
                            shape: action_focus::foreground(side),
                            on_click: None,
                        },
                    ]
                }
                NzscAction::Move(move_) => vec![
                    Component::Circle {
                        fill_color: colors::move_color(move_),
                        shape: action_focus::background(side),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::Move(move_),
                        alpha: 1.0,
                        shape: action_focus::foreground(side),
                        on_click: None,
                    },
                ],
            }
        })
        .into_iter()
        .flatten()
        .collect()
}

fn fading_action(side: Side, visit: Option<ActionVisit>, lerper: &Lerper) -> Vec<Component> {
    let sublerper = lerper.sub_lerper(0.0..colors::PORTION_OF_DURATION_SPENT_FADING);

    visit
        .map(|visit| {
            let ActionVisit { action, .. } = visit;

            match action {
                NzscAction::Concede => panic!("optional visit should be None if Action is Concede"),
                NzscAction::Mirror(move_) => {
                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_).with_alpha(0),
                            start_shape: action_focus::background(side),
                            end_shape: action_focus::background(side),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Mirror,
                            start_alpha: 1.0,
                            end_alpha: 0.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: action_focus::foreground(side),
                            on_click: None,
                        },
                        // TODO Shrink move image
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 0.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: action_focus::foreground(side),
                            on_click: None,
                        },
                    ]
                }
                NzscAction::Move(move_) => vec![
                    LerpableComponent::Circle {
                        start_color: colors::move_color(move_),
                        end_color: colors::move_color(move_).with_alpha(0),
                        start_shape: action_focus::background(side),
                        end_shape: action_focus::background(side),
                        on_click: None,
                    },
                    LerpableComponent::Image {
                        image_type: ImageType::Move(move_),
                        start_alpha: 1.0,
                        end_alpha: 0.0,
                        start_shape: action_focus::foreground(side),
                        end_shape: action_focus::foreground(side),
                        on_click: None,
                    },
                ],
            }
        })
        .into_iter()
        .flatten()
        .map(|lerpable: LerpableComponent| sublerper.lerp1(lerpable))
        .collect()
}

fn exiting_action(side: Side, visit: Option<ActionVisit>, lerper: &Lerper) -> Vec<Component> {
    if let Some(visit) = visit {
        let ActionVisit { action, end, .. } = visit;
        match action {
            NzscAction::Concede => panic!("optional visit should be None if Action is Concede"),

            NzscAction::Mirror(move_) => {
                if let Some(end) = end {
                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_),
                            start_shape: action_focus::background(side),
                            end_shape: dequeue_circle::background_at(end.side, end.row, end.column),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Mirror,
                            start_alpha: 1.0,
                            end_alpha: 1.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: dequeue_circle::foreground_at(end.side, end.row, end.column),
                            on_click: None,
                        },
                        // TODO Shrink move image
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 1.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: dequeue_circle::foreground_at(end.side, end.row, end.column),
                            on_click: None,
                        },
                    ]
                    .into_iter()
                    .map(|lerpable| lerper.lerp1(lerpable))
                    .collect()
                } else {
                    let sublerper = lerper
                        .sub_lerper(0.0..colors::PORTION_OF_DURATION_DESTROYED_ITEM_SPENDS_POPPING);

                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_).with_alpha(0),
                            start_shape: action_focus::background(side),
                            end_shape: action_focus::expanded_background(side),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Mirror,
                            start_alpha: 1.0,
                            end_alpha: 0.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: action_focus::expanded_foreground(side),
                            on_click: None,
                        },
                        // TODO Shrink move image
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 0.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: action_focus::expanded_foreground(side),
                            on_click: None,
                        },
                    ]
                    .into_iter()
                    .map(|lerpable| sublerper.lerp1(lerpable))
                    .collect()
                }
            }

            NzscAction::Move(move_) => {
                if let Some(end) = end {
                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_),
                            start_shape: action_focus::background(side),
                            end_shape: dequeue_circle::background_at(end.side, end.row, end.column),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 1.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: dequeue_circle::foreground_at(end.side, end.row, end.column),
                            on_click: None,
                        },
                    ]
                    .into_iter()
                    .map(|lerpable| lerper.lerp1(lerpable))
                    .collect()
                } else {
                    let sublerper = lerper
                        .sub_lerper(0.0..colors::PORTION_OF_DURATION_DESTROYED_ITEM_SPENDS_POPPING);

                    vec![
                        LerpableComponent::Circle {
                            start_color: colors::move_color(move_),
                            end_color: colors::move_color(move_).with_alpha(0),
                            start_shape: action_focus::background(side),
                            end_shape: action_focus::expanded_background(side),
                            on_click: None,
                        },
                        LerpableComponent::Image {
                            image_type: ImageType::Move(move_),
                            start_alpha: 1.0,
                            end_alpha: 0.0,
                            start_shape: action_focus::foreground(side),
                            end_shape: action_focus::expanded_foreground(side),
                            on_click: None,
                        },
                    ]
                    .into_iter()
                    .map(|lerpable| sublerper.lerp1(lerpable))
                    .collect()
                }
            }
        }
    } else {
        vec![]
    }
}

fn action_choosing_scoreboard_without_used_item(
    args: ActionChoosingRenderArgs,
    used_item: Option<ArsenalItem>,
) -> Vec<Component> {
    vec![
        action_choosing_pool_display(&args),
        action_choosing_entrance_and_exit_display(&args),
        action_choosing_arsenal_display_without_used_item(&args, used_item),
        arrows(&args),
    ]
    .into_iter()
    .flatten()
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

fn dequeueing_pool_display(args: &DequeueingRenderArgs) -> Vec<Component> {
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
            side,
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
            arsenal_item_display(
                arsenal_item,
                drain_and_exit_enabled,
                side.if_left(Action::ChooseDequeue(DequeueChoice::DrainAndExit(
                    arsenal_item,
                ))),
                CirclePosition {
                    side,
                    column: i % 3,
                    row: i / 3,
                },
            )
        });

    pill.render(()).into_iter().chain(pool).collect()
}

fn dequeueing_entrance_decline_and_exit_display(args: &DequeueingRenderArgs) -> Vec<Component> {
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
            side,
            column: 0,
            row: pool_height_in_rows,
        },
        width_in_columns: 3,
        height_in_rows: 1,
        enabled: false,
    };
    let decline_and_exit_pill = Pill {
        position: CirclePosition {
            side,
            column: 1,
            row: pool_height_in_rows,
        },
        width_in_columns: 2,
        height_in_rows: 1,
        enabled: true,
    };
    let exit_pill = Pill {
        position: CirclePosition {
            side,
            column: 2,
            row: pool_height_in_rows,
        },
        width_in_columns: 1,
        height_in_rows: 1,
        enabled: just_exit_enabled,
    };

    vec![
        Some(background_pill.render(())),
        Some(decline_and_exit_pill.render(())),
        Some(exit_pill.render(())),
        entrance.map(|entrance| {
            arsenal_item_display(
                entrance,
                false,
                None,
                CirclePosition {
                    side,
                    column: 0,
                    row,
                },
            )
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
        exit.map(|exiting_item| {
            arsenal_item_display(
                exiting_item,
                just_exit_enabled,
                side.if_left(Action::ChooseDequeue(DequeueChoice::JustExit)),
                CirclePosition {
                    side,
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

fn dequeueing_arsenal_display(args: &DequeueingRenderArgs) -> Vec<Component> {
    let DequeueingRenderArgs { player, side, .. } = args;
    let side = *side;

    let row_offset = helpers::height_in_rows(&player.queue.pool, 3) + 1;

    let pill = Pill {
        position: CirclePosition {
            side,
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
                CirclePosition { side, column, row },
            )
        });

    pill.render(()).into_iter().chain(arsenal_items).collect()
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
            side,
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
                        fill_color: colors::move_color(move_),
                        shape: dequeue_circle::background_at(side, row, column),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::Mirror,
                        alpha: 1.0,
                        shape: dequeue_circle::foreground_at(side, row, column),
                        on_click: None,
                    },
                    Component::Image {
                        image_type: ImageType::Move(move_),
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
                    CirclePosition { side, column, row },
                )
            }
        });

    pill.render(()).into_iter().chain(pool).collect()
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
            side,
            column: 0,
            row: pool_height_in_rows,
        },
        width_in_columns: 3,
        height_in_rows: 1,
        enabled: false,
    };

    vec![
        Some(pill.render(())),
        entrance.map(|entering_item| {
            arsenal_item_display(
                entering_item,
                false,
                None,
                CirclePosition {
                    side,
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
                    side,
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
    action_choosing_arsenal_display_without_used_item(args, None)
}

fn action_choosing_arsenal_display_without_used_item(
    args: &ActionChoosingRenderArgs,
    used_item: Option<ArsenalItem>,
) -> Vec<Component> {
    let ActionChoosingRenderArgs { player, side, .. } = args;
    let side = *side;

    let row_offset = helpers::height_in_rows(&player.queue.pool, 3) + 1;

    let pill = Pill {
        position: CirclePosition {
            side,
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
            if Some(arsenal_item) == used_item {
                vec![]
            } else {
                let row = i / 3;
                let column = i % 3;
                let row = row + row_offset;

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
            }
        });

    pill.render(()).into_iter().chain(arsenal_items).collect()
}

fn position_of(item: ArsenalItem, player: &QueueArsenal, side: Side) -> Option<CirclePosition> {
    let index = player
        .queue()
        .pool
        .iter()
        .position(|&pool_item| pool_item == item);
    if let Some(index) = index {
        Some(CirclePosition {
            side,
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
            side,
            column: 0,
            row: pool_height,
        })
    } else if Some(item) == player.queue().exit {
        Some(CirclePosition {
            side,
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
            side,
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

struct ActionChoosingRenderArgs<'a> {
    pub player: &'a ActionlessPlayer,
    pub side: Side,
    pub actions: &'a Vec<NzscAction>,
}

struct DequeueingRenderArgs<'a> {
    pub player: &'a DequeueingPlayer,
    pub side: Side,
    pub dequeues: &'a Vec<DequeueChoice>,
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

struct ActionVisit {
    pub action: NzscAction,
    pub start: CirclePosition,
    pub end: Option<CirclePosition>,
}

const HUMAN: usize = 0;
const COMPUTER: usize = 1;
