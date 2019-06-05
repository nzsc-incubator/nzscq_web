use crate::{
    click::Action,
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

    pub fn sub_lerper<T: std::ops::RangeBounds<f64>>(&self, range: T) -> Lerper {
        use std::ops::Bound;

        let completion_factor = self.0;
        
        let min = match range.start_bound() {
            Bound::Included(min) => min,
            Bound::Excluded(min) => min,
            Bound::Unbounded => panic!("start bound required")
        };
        let max = match range.end_bound() {
            Bound::Included(min) => min,
            Bound::Excluded(min) => min,
            Bound::Unbounded => panic!("start bound required")
        };
        let new_completion_factor = (completion_factor - min) / (max - min);
        Lerper(new_completion_factor)
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
        on_click: Option<Action>,
    },
    Circle {
        fill_color: &'static str,
        start: Circle,
        end: Circle,
        on_click: Option<Action>,
    },
    Image {
        image_type: ImageType,
        start: Rect,
        end: Rect,
        on_click: Option<Action>,
    },
}

impl LerpInto<Component> for Lerpable {
    fn lerp(self, lerper: &Lerper) -> Component {
        match self {
            Lerpable::Rect {
                fill_color,
                start,
                end,
                on_click,
            } => Component::Rect {
                fill_color,
                shape: (start, end).lerp(lerper),
                on_click,
            },

            Lerpable::Circle {
                fill_color,
                start,
                end,
                on_click,
            } => Component::Circle {
                fill_color,
                shape: (start, end).lerp(lerper),
                on_click,
            },

            Lerpable::Image {
                image_type,
                start,
                end,
                on_click,
            } => Component::Image {
                image_type,
                shape: (start, end).lerp(lerper),
                on_click,
            },
        }
    }
}
