use crate::colors;
use crate::helpers::SQRT_3;
use crate::paint::{Component, Path, PathCommand};
use crate::shapes::{
    dequeue_circle::{self, LEFT_COLUMN_0_X, RIGHT_COLUMN_0_X},
    Translate,
};
use crate::side::Side;

pub fn up_arrow_above(side: Side, row: usize, column: usize) -> Component {
    match side {
        Side::Left => left_up_arrow_above(row, column),
        Side::Right => right_up_arrow_above(row, column),
    }
}

fn left_up_arrow_above(row: usize, column: usize) -> Component {
    Component::UnclickablePath {
        path: Path {
            start: (LEFT_COLUMN_0_X, ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT),
            commands: vec![
                PathCommand::LineTo(
                    LEFT_COLUMN_0_X + 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT,
                ),
                PathCommand::LineTo(
                    LEFT_COLUMN_0_X - 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT,
                ),
            ],
        }
        .translate(OFFSET * column as f64, OFFSET * row as f64),
        fill_color: Some(colors::ARROW_COLOR),
        stroke: None,
    }
}

fn right_up_arrow_above(row: usize, column: usize) -> Component {
    Component::UnclickablePath {
        path: Path {
            start: (RIGHT_COLUMN_0_X, ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT),
            commands: vec![
                PathCommand::LineTo(
                    RIGHT_COLUMN_0_X + 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT,
                ),
                PathCommand::LineTo(
                    RIGHT_COLUMN_0_X - 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT,
                ),
            ],
        }
        .translate(-OFFSET * column as f64, OFFSET * row as f64),
        fill_color: Some(colors::ARROW_COLOR),
        stroke: None,
    }
}

pub fn down_arrow_above(side: Side, row: usize, column: usize) -> Component {
    match side {
        Side::Left => left_down_arrow_above(row, column),
        Side::Right => right_down_arrow_above(row, column),
    }
}

fn left_down_arrow_above(row: usize, column: usize) -> Component {
    Component::UnclickablePath {
        path: Path {
            start: (LEFT_COLUMN_0_X, ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT),
            commands: vec![
                PathCommand::LineTo(
                    LEFT_COLUMN_0_X + 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT,
                ),
                PathCommand::LineTo(
                    LEFT_COLUMN_0_X - 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT,
                ),
            ],
        }
        .translate(OFFSET * column as f64, OFFSET * row as f64),

        fill_color: Some(colors::ARROW_COLOR),
        stroke: None,
    }
}

fn right_down_arrow_above(row: usize, column: usize) -> Component {
    Component::UnclickablePath {
        path: Path {
            start: (RIGHT_COLUMN_0_X, ROW_0_CENTER_Y + 0.5 * ARROW_HEIGHT),
            commands: vec![
                PathCommand::LineTo(
                    RIGHT_COLUMN_0_X + 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT,
                ),
                PathCommand::LineTo(
                    RIGHT_COLUMN_0_X - 0.5 * ARROW_WIDTH,
                    ROW_0_CENTER_Y - 0.5 * ARROW_HEIGHT,
                ),
            ],
        }
        .translate(-OFFSET * column as f64, OFFSET * row as f64),

        fill_color: Some(colors::ARROW_COLOR),
        stroke: None,
    }
}

const ROW_0_CENTER_Y: f64 = dequeue_circle::ROW_0_Y + 110.0 - OFFSET;
const ARROW_HEIGHT: f64 = 40.0;
const ARROW_WIDTH: f64 = ARROW_HEIGHT * 2.0 / SQRT_3;
const OFFSET: f64 = dequeue_circle::DIAMETER + dequeue_circle::MARGIN;
