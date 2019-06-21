#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct CenteredRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Into<Rect> for CenteredRect {
    fn into(self) -> Rect {
        Rect {
            x: self.x - self.width / 2.0,
            y: self.y - self.height / 2.0,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub trait Translate {
    fn translate(&self, x: f64, y: f64) -> Self;
}

impl Translate for Rect {
    fn translate(&self, x: f64, y: f64) -> Rect {
        Rect {
            x: self.x + x,
            y: self.y + y,
            ..*self
        }
    }
}

impl Translate for Circle {
    fn translate(&self, x: f64, y: f64) -> Circle {
        Circle {
            x: self.x + x,
            y: self.y + y,
            ..*self
        }
    }
}

pub mod rect_button {
    use super::Rect;
    use crate::canvas_dimensions;

    const WIDTH: f64 = 400.0;
    const HEIGHT: f64 = 800.0;
    const H_MARGIN: f64 = 40.0;
    const V_MARGIN: f64 = (canvas_dimensions::HEIGHT - HEIGHT) / 2.0;

    pub fn background_at(index: usize) -> Rect {
        Rect {
            width: WIDTH,
            height: HEIGHT,
            x: H_MARGIN + index as f64 * (WIDTH + H_MARGIN),
            y: V_MARGIN,
        }
    }

    pub fn foreground_at(index: usize) -> Rect {
        Rect {
            width: WIDTH,
            height: WIDTH,
            x: background_at(index).x,
            y: V_MARGIN + (HEIGHT - WIDTH) / 2.0,
        }
    }
}

pub mod rect_focus {
    use super::{Rect, Translate};
    use crate::canvas_dimensions;

    const H_MARGIN: f64 = 200.0;
    const LEFT_X: f64 = H_MARGIN;
    const RIGHT_INITIAL_X: f64 = canvas_dimensions::WIDTH;
    const RIGHT_FINAL_X: f64 = canvas_dimensions::WIDTH - H_MARGIN - WIDTH;
    const V_MARGIN: f64 = (canvas_dimensions::HEIGHT - HEIGHT) / 2.0;
    const WIDTH: f64 = 600.0;
    const HEIGHT: f64 = 800.0;

    pub fn left_background() -> Rect {
        Rect {
            x: LEFT_X,
            y: V_MARGIN,
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn left_foreground() -> Rect {
        Rect {
            x: LEFT_X,
            y: V_MARGIN + (HEIGHT - WIDTH) / 2.0,
            width: WIDTH,
            height: WIDTH,
        }
    }

    pub fn far_left_background() -> Rect {
        let left_bg = left_background();

        left_bg.translate(-(left_bg.x + left_bg.width), 0.0)
    }

    pub fn far_left_foreground() -> Rect {
        let left_fg = left_foreground();

        left_fg.translate(-(left_fg.x + left_fg.width), 0.0)
    }

    pub fn right_background() -> Rect {
        Rect {
            x: RIGHT_FINAL_X,
            y: V_MARGIN,
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn right_foreground() -> Rect {
        Rect {
            x: RIGHT_FINAL_X,
            y: V_MARGIN + (HEIGHT - WIDTH) / 2.0,
            width: WIDTH,
            height: WIDTH,
        }
    }

    pub fn far_right_background() -> Rect {
        let right_bg = right_background();

        right_bg.translate(1800.0 - right_bg.x, 0.0)
    }

    pub fn far_right_foreground() -> Rect {
        let right_fg = right_foreground();

        right_fg.translate(1800.0 - right_fg.x, 0.0)
    }
}

pub mod dequeue_circle {
    use super::{Circle, Rect};
    use crate::canvas_dimensions;
    use crate::side::Side;

    const TRAPEZOID_BOTTOM: f64 = 92.0;
    const RADIUS: f64 = 100.0;
    pub const DIAMETER: f64 = 2.0 * RADIUS;
    pub const MARGIN: f64 =
        ((canvas_dimensions::HEIGHT - TRAPEZOID_BOTTOM) - (4.0 * DIAMETER)) / 5.0;
    pub const LEFT_COLUMN_0_X: f64 = 120.0;
    pub const RIGHT_COLUMN_0_X: f64 = canvas_dimensions::WIDTH - 120.0;
    pub const ROW_0_Y: f64 = TRAPEZOID_BOTTOM + MARGIN + RADIUS;
    pub const OFFSET: f64 = DIAMETER + MARGIN;

    pub fn background_at(side: Side, row: usize, column: usize) -> Circle {
        match side {
            Side::Left => left_background_at(row, column),
            Side::Right => right_background_at(row, column),
        }
    }

    fn left_background_at(row: usize, column: usize) -> Circle {
        Circle {
            x: LEFT_COLUMN_0_X + (OFFSET) * column as f64,
            y: ROW_0_Y + (OFFSET) * row as f64,
            radius: RADIUS,
        }
    }

    fn right_background_at(row: usize, column: usize) -> Circle {
        Circle {
            x: RIGHT_COLUMN_0_X - (OFFSET) * column as f64,
            y: ROW_0_Y + (OFFSET) * row as f64,
            radius: RADIUS,
        }
    }

    pub fn foreground_at(side: Side, row: usize, column: usize) -> Rect {
        match side {
            Side::Left => left_foreground_at(row, column),
            Side::Right => right_foreground_at(row, column),
        }
    }

    fn left_foreground_at(row: usize, column: usize) -> Rect {
        let bg = left_background_at(row, column);

        Rect {
            x: bg.x - RADIUS,
            y: bg.y - RADIUS,
            width: DIAMETER,
            height: DIAMETER,
        }
    }

    fn right_foreground_at(row: usize, column: usize) -> Rect {
        let bg = right_background_at(row, column);

        Rect {
            x: bg.x - RADIUS,
            y: bg.y - RADIUS,
            width: DIAMETER,
            height: DIAMETER,
        }
    }

    #[derive(Debug, Clone)]
    pub struct CirclePosition {
        pub from: Side,
        pub column: usize,
        pub row: usize,
    }

    impl CirclePosition {
        pub fn x(&self) -> f64 {
            match self.from {
                Side::Left => LEFT_COLUMN_0_X + OFFSET * self.column as f64,
                Side::Right => RIGHT_COLUMN_0_X - OFFSET * self.column as f64,
            }
        }

        pub fn y(&self) -> f64 {
            ROW_0_Y + OFFSET * self.row as f64
        }
    }
}
