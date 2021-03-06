use crate::click::Action;
use crate::colors::{self, Rgba};
use crate::helpers::{self, QueueArsenal};
use crate::paint::{Component, ImageType};
use crate::render::{arsenal_item_display, pill::Pill, Render};
use crate::shapes::{
    dequeue_circle::{self, CirclePosition},
    move_inspector_highlighter,
};
use crate::side::Side;

use nzscq::choices::{ArsenalItem, Move, PointsAgainst};
use nzscq::scoreboard::Queue;

pub struct MoveInspector<'a> {
    side: Side,
    queue: &'a Queue,
    arsenal: &'a Vec<ArsenalItem>,
    inspected_move: Option<Move>,
}

impl<'a> MoveInspector<'a> {
    pub fn new(args: MoveInspectorArgs) -> MoveInspector {
        MoveInspector {
            side: args.side,
            queue: args.player.queue(),
            arsenal: args.player.arsenal(),
            inspected_move: args.inspected_move,
        }
    }

    fn render_pool(&self) -> Vec<Component> {
        let pill = Pill {
            position: CirclePosition {
                side: self.side,
                column: 0,
                row: 0,
            },
            width_in_columns: 3,
            height_in_rows: helpers::height_in_rows(&self.queue.pool, 3),
            enabled: true,
        };

        let pool = self
            .queue
            .pool
            .iter()
            .enumerate()
            .flat_map(|(i, &arsenal_item)| {
                let row = i / 3;
                let column = i % 3;
                let position = CirclePosition {
                    side: self.side,
                    column,
                    row,
                };

                self.highlighter(arsenal_item, position)
                    .into_iter()
                    .chain(arsenal_item_display(
                        arsenal_item,
                        true,
                        inspection_handler(arsenal_item),
                        position,
                    ))
            });

        pill.render(()).into_iter().chain(pool).collect()
    }

    fn render_entrance_and_exit(&self) -> Vec<Component> {
        let entrance = self.queue.entrance;
        let exit = self.queue.exit;
        let pool_height_in_rows = helpers::height_in_rows(&self.queue.pool, 3);
        let row = pool_height_in_rows;

        let background_pill = Pill {
            position: CirclePosition {
                side: self.side,
                column: 0,
                row: pool_height_in_rows,
            },
            width_in_columns: 3,
            height_in_rows: 1,
            enabled: true,
        };

        vec![
            Some(background_pill.render(())),
            entrance.map(|entering_item| {
                let position = CirclePosition {
                    side: self.side,
                    column: 0,
                    row,
                };

                self.highlighter(entering_item, position)
                    .into_iter()
                    .chain(arsenal_item_display(
                        entering_item,
                        true,
                        inspection_handler(entering_item),
                        position,
                    ))
                    .collect()
            }),
            exit.map(|exiting_item| {
                let position = CirclePosition {
                    side: self.side,
                    column: 2,
                    row,
                };

                self.highlighter(exiting_item, position)
                    .into_iter()
                    .chain(arsenal_item_display(
                        exiting_item,
                        true,
                        inspection_handler(exiting_item),
                        position,
                    ))
                    .collect()
            }),
        ]
        .into_iter()
        .flatten()
        .flatten()
        .collect()
    }

    fn render_arsenal(&self) -> Vec<Component> {
        let row_offset = helpers::height_in_rows(&self.queue.pool, 3) + 1;

        let pill = Pill {
            position: CirclePosition {
                side: self.side,
                column: 0,
                row: row_offset,
            },
            width_in_columns: 3,
            height_in_rows: helpers::height_in_rows(&self.arsenal, 3).max(1),
            enabled: true,
        };
        let arsenal_items = self
            .arsenal
            .iter()
            .enumerate()
            .flat_map(|(i, &arsenal_item)| {
                let row = i / 3;
                let column = i % 3;
                let row = row + row_offset;
                let position = CirclePosition {
                    side: self.side,
                    column,
                    row,
                };

                self.highlighter(arsenal_item, position)
                    .into_iter()
                    .chain(arsenal_item_display(
                        arsenal_item,
                        true,
                        inspection_handler(arsenal_item),
                        position,
                    ))
            });

        pill.render(()).into_iter().chain(arsenal_items).collect()
    }

    fn highlighter(
        &self,
        opposing_arsenal_item: ArsenalItem,
        position: CirclePosition,
    ) -> Option<Component> {
        if let ArsenalItem::Move(opposing_move) = opposing_arsenal_item {
            self.highlighter_color(opposing_move)
                .map(|fill_color| Component::Circle {
                    fill_color,
                    shape: move_inspector_highlighter::circle_at(position),
                    on_click: None,
                })
        } else {
            None
        }
    }

    fn highlighter_color(&self, opposing_move: Move) -> Option<Rgba> {
        if self.inspected_move == Some(opposing_move) {
            Some(colors::INSPECTED_MOVE_HIGHLIGHT_COLOR)
        } else if let Some(inspected_move) = self.inspected_move {
            let points = PointsAgainst::points_of(&[inspected_move, opposing_move]);

            colors::move_inspector_highlighter_color(points[0], points[1])
        } else {
            None
        }
    }
}

impl<'a> Render<()> for MoveInspector<'a> {
    fn render(&self, _: ()) -> Vec<Component> {
        vec![
            self.render_pool(),
            self.render_entrance_and_exit(),
            self.render_arsenal(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

pub struct MoveInspectorArgs<'a> {
    pub side: Side,
    pub player: &'a QueueArsenal,
    pub inspected_move: Option<Move>,
}

fn inspection_handler(arsenal_item: ArsenalItem) -> Option<Action> {
    if let ArsenalItem::Move(m) = arsenal_item {
        Some(Action::InspectMove(m))
    } else {
        None
    }
}
