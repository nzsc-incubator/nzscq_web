use crate::colors;
use crate::paint::Component;
use crate::shapes::{dequeue_circle, Circle, Rect, Translate};

pub struct DequeueBackground {
    pub x: f64,
    pub y: f64,
}

impl DequeueBackground {
    pub fn left_at(row: usize) -> DequeueBackground {
        DequeueBackground {
            x: 120.0,
            y: 213.6 + (dequeue_circle::DIAMETER + dequeue_circle::MARGIN) * row as f64,
        }
    }

    pub fn n_rows(&self, n: usize) -> Vec<Component> {
        match n {
            0 => vec![],
            1 => self.one_row(),
            2 => self.two_rows(),
            _ => panic!("n must be 0, 1, or 2"),
        }
    }

    pub fn one_row(&self) -> Vec<Component> {
        let rect = Component::Rect {
            fill_color: colors::DEQUEUE_BACKGROUND_COLOR,
            shape: Rect {
                x: 0.0,
                y: -110.0,
                width: 443.2,
                height: 220.0,
            }
            .translate(self.x, self.y),
            on_click: None,
        };

        let top_left_circle_shape = Circle {
            x: 0.0,
            y: 0.0,
            radius: 110.0,
        };
        let top_right_circle_shape = Circle {
            x: 443.2,
            y: 0.0,
            radius: 110.0,
        };
        let circles = vec![top_left_circle_shape, top_right_circle_shape]
            .into_iter()
            .map(|shape| Component::Circle {
                fill_color: colors::DEQUEUE_BACKGROUND_COLOR,
                shape: shape.translate(self.x, self.y),
                on_click: None,
            });

        circles.chain(vec![rect]).collect()
    }

    pub fn two_rows(&self) -> Vec<Component> {
        let top_rect_shape = Rect {
            x: 0.0,
            y: -110.0,
            width: 443.2,
            height: 220.0,
        };
        let middle_rect_shape = Rect {
            x: -110.0,
            y: 0.0,
            width: 663.2,
            height: 220.0,
        };
        let bottom_rect_shape = Rect {
            x: 0.0,
            y: 111.6,
            width: 443.2,
            height: 220.0,
        };
        let rects = vec![top_rect_shape, middle_rect_shape, bottom_rect_shape]
            .into_iter()
            .map(|shape| Component::Rect {
                fill_color: colors::DEQUEUE_BACKGROUND_COLOR,
                shape: shape.translate(self.x, self.y),

                on_click: None,
            });

        let top_left_circle_shape = Circle {
            x: 0.0,
            y: 0.0,
            radius: 110.0,
        };
        let top_right_circle_shape = Circle {
            x: 443.2,
            y: 0.0,
            radius: 110.0,
        };
        let bottom_left_circle_shape = Circle {
            x: 0.0,
            y: 221.6,
            radius: 110.0,
        };
        let bottom_right_circle_shape = Circle {
            x: 443.2,
            y: 221.6,
            radius: 110.0,
        };
        let circles = vec![
            top_left_circle_shape,
            top_right_circle_shape,
            bottom_left_circle_shape,
            bottom_right_circle_shape,
        ]
        .into_iter()
        .map(|shape| Component::Circle {
            fill_color: colors::DEQUEUE_BACKGROUND_COLOR,
            shape: shape.translate(self.x, self.y),
            on_click: None,
        });

        rects.chain(circles).collect()
    }
}
