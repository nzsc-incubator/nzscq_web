use crate::{
    click::Action,
    paint::{Component, ImageType},
    shapes::{Circle, Rect},
    colors::Rgba,
};

pub struct Lerper(f64);

impl Lerper {
    pub fn from_completion_factor(factor: f64) -> Lerper {
        Lerper(factor)
    }

    pub fn lerp<T, U>(&self, start: T, end: T)  -> U where (T, T): LerpInto<U>{
        (start, end).lerp_into(self)
    }

    pub fn lerp1<T, U>(&self, lerpable: T) -> U where T: LerpInto<U> {
        lerpable.lerp_into(self)
    }

    pub fn sub_lerper<T: std::ops::RangeBounds<f64>>(&self, range: T) -> Lerper {
        use std::ops::Bound;

        let completion_factor = self.0;

        let min = match range.start_bound() {
            Bound::Included(min) => min,
            Bound::Excluded(min) => min,
            Bound::Unbounded => panic!("start bound required"),
        };
        let max = match range.end_bound() {
            Bound::Included(min) => min,
            Bound::Excluded(min) => min,
            Bound::Unbounded => panic!("start bound required"),
        };
        let new_completion_factor = (completion_factor - min) / (max - min);
        Lerper(new_completion_factor)
    }
}

pub trait LerpInto<T> {
    fn lerp_into(self, lerper: &Lerper) -> T;
}

impl LerpInto<f64> for (f64, f64) {
    fn lerp_into(self, lerper: &Lerper) -> f64 {
        let (start, end) = self;

        start + (end - start) * lerper.0
    }
}

impl LerpInto<u8> for (u8, u8) {
    fn lerp_into(self, lerper: &Lerper) -> u8 {
        let (start, end) = self;

        lerper.lerp(start as f64, end as f64) as u8
    }
}

impl LerpInto<Rgba> for (Rgba, Rgba) {
    fn lerp_into(self, lerper: &Lerper) -> Rgba {
        let (
            Rgba(r0, g0, b0, a0),
            Rgba(r1, g1, b1, a1)
        ) = self;

        Rgba(lerper.lerp(r0, r1), lerper.lerp(g0, g1), lerper.lerp(b0, b1), lerper.lerp(a0, a1))
    }
}

impl LerpInto<Rect> for (Rect, Rect) {
    fn lerp_into(self, lerper: &Lerper) -> Rect {
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
    fn lerp_into(self, lerper: &Lerper) -> Circle {
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
pub enum LerpableComponent {
    Rect {
        start_color: Rgba,
        end_color: Rgba,
        start_shape: Rect,
        end_shape: Rect,
        on_click: Option<Action>,
    },
    Circle {
        start_color: Rgba,
        end_color: Rgba,
        start_shape: Circle,
        end_shape: Circle,
        on_click: Option<Action>,
    },
    Image {
        image_type: ImageType,
        start_alpha: f64,
        end_alpha: f64,
        start_shape: Rect,
        end_shape: Rect,
        on_click: Option<Action>,
    },
}

impl LerpInto<Component> for LerpableComponent {
    fn lerp_into(self, lerper: &Lerper) -> Component {
        match self {
            LerpableComponent::Rect {
                start_color,
                end_color,
                start_shape,
                end_shape,
                on_click,
            } => Component::Rect {
                fill_color: lerper.lerp(start_color, end_color),
                shape: lerper.lerp(start_shape, end_shape),
                on_click,
            },

            LerpableComponent::Circle {
                start_color,
                end_color,
                start_shape,
                end_shape,
                on_click,
            } => Component::Circle {
                fill_color: lerper.lerp(start_color, end_color),
                shape: lerper.lerp(start_shape, end_shape),
                on_click,
            },

            LerpableComponent::Image {
                image_type,
                start_alpha,
                end_alpha,
                start_shape,
                end_shape,
                on_click,
            } => Component::Image {
                image_type,
                alpha: lerper.lerp(start_alpha, end_alpha),
                shape: lerper.lerp(start_shape, end_shape),
                on_click,
            },
        }
    }
}
