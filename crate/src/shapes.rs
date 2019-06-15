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
