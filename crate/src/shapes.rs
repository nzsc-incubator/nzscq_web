#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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

    const WIDTH: f64 = 400.0;
    const HEIGHT: f64 = 800.0;
    const H_MARGIN: f64 = 40.0;
    const V_MARGIN: f64 = 100.0;

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
