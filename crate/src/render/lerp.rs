use crate::{
    paint::{Component, ImageType},
    shapes::{Circle, Rect},
};

pub struct Lerper(f64);

impl Lerper {
    pub fn from_completion_factor(factor: f64) -> Lerper {
        Lerper(factor)
    }

    pub fn lerp(&self, start: f64, end: f64) -> f64 {
        start + (end - start) * self.0
    }
}

pub trait LerpInto<T> {
    fn lerp(self, lerper: &Lerper) -> T;
}

impl LerpInto<Rect> for (Rect, Rect) {
    fn lerp(self, lerper: &Lerper) -> Rect {
        let (start, end) = self;
        let (x, y, width, height) = (
            lerper.lerp(start.x, end.x),
            lerper.lerp(start.y, end.y),
            lerper.lerp(start.width, end.width),
            lerper.lerp(start.height, end.height),
        );

        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

impl LerpInto<Circle> for (Circle, Circle) {
    fn lerp(self, lerper: &Lerper) -> Circle {
        let (start, end) = self;
        let (x, y, radius) = (
            lerper.lerp(start.x, end.x),
            lerper.lerp(start.y, end.y),
            lerper.lerp(start.radius, end.radius),
        );

        Circle { x, y, radius }
    }
}

#[derive(Debug, Clone)]
pub enum Lerpable {
    Rect {
        fill_color: &'static str,
        start: Rect,
        end: Rect,
    },
    Circle {
        fill_color: &'static str,
        start: Circle,
        end: Circle,
    },
    Image {
        image_type: ImageType,
        start: Rect,
        end: Rect,
    },
}

impl LerpInto<Component> for Lerpable {
    fn lerp(self, lerper: &Lerper) -> Component {
        match self {
            Lerpable::Rect {
                fill_color,
                start,
                end,
            } => Component::Rect {
                fill_color,
                shape: (start, end).lerp(lerper),
            },

            Lerpable::Circle {
                fill_color,
                start,
                end,
            } => Component::Circle {
                fill_color,
                shape: (start, end).lerp(lerper),
            },

            Lerpable::Image {
                image_type,
                start,
                end,
            } => Component::Image {
                image_type,
                shape: (start, end).lerp(lerper),
            },
        }
    }
}
