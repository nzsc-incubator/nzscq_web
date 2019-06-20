use super::Render;
use crate::colors::{self, Rgba};
use crate::paint::Component;
use crate::shapes::{
    dequeue_circle::{CirclePosition, OFFSET},
    Circle, Rect, Translate,
};

pub struct Pill {
    pub position: CirclePosition,
    pub width_in_columns: usize,
    pub height_in_rows: usize,
    pub enabled: bool,
}

impl Pill {
    fn one_row(&self) -> Vec<Component> {
        let rect = Component::Rect {
            fill_color: self.color(),
            shape: Rect {
                x: 0.0,
                y: -110.0,
                width: 443.2,
                height: 220.0,
            }
            .translate(self.x(), self.y()),
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
                fill_color: self.color(),
                shape: shape.translate(self.x(), self.y()),
                on_click: None,
            });

        circles.chain(vec![rect]).collect()
    }

    fn two_rows(&self) -> Vec<Component> {
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
                fill_color: self.color(),
                shape: shape.translate(self.x(), self.y()),

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
            fill_color: self.color(),
            shape: shape.translate(self.x(), self.y()),
            on_click: None,
        });

        rects.chain(circles).collect()
    }

    fn x(&self) -> f64 {
        self.position.x()
    }

    fn y(&self) -> f64 {
        self.position.y()
    }

    fn color(&self) -> Rgba {
        if self.enabled {
            colors::PILL_ENABLED_COLOR
        } else {
            colors::PILL_DISABLED_COLOR
        }
    }
}

impl Render for Pill {
    // fn render(&self) -> Vec<Component> {
    //     match self.height_in_rows {
    //         0 => vec![],
    //         1 => self.one_row(),
    //         2 => self.two_rows(),
    //         _ => panic!("n must be 0, 1, or 2"),
    //     }
    // }

    fn render(&self) -> Vec<Component> {
        let vertical_connector = Rect {
            x: 0.0,
            y: -RADIUS,
            width: (OFFSET * (self.width_in_columns - 1) as f64),
            height: (2.0 * RADIUS) + (OFFSET * (self.height_in_rows - 1) as f64),
        };
        let horizontal_connector = Rect {
            x: -RADIUS,
            y: 0.0,
            width: (2.0 * RADIUS) + (OFFSET * (self.width_in_columns - 1) as f64),
            height: if self.height_in_rows < 2 {
                0.0
            } else {
                (2.0 * RADIUS) + (OFFSET * (self.height_in_rows - 2) as f64)
            },
        };
        let rects = vec![vertical_connector, horizontal_connector]
            .into_iter()
            .map(|shape| Component::Rect {
                fill_color: self.color(),
                shape: shape.translate(self.x(), self.y()),

                on_click: None,
            });

        let top_left_circle_shape = Circle {
            x: 0.0,
            y: 0.0,
            radius: RADIUS,
        };
        let top_right_circle_shape = Circle {
            x: OFFSET * (self.width_in_columns - 1) as f64,
            y: 0.0,
            radius: RADIUS,
        };
        let bottom_left_circle_shape = Circle {
            x: 0.0,
            y: OFFSET * (self.height_in_rows - 1) as f64,
            radius: RADIUS,
        };
        let bottom_right_circle_shape = Circle {
            x: OFFSET * (self.width_in_columns - 1) as f64,
            y: OFFSET * (self.height_in_rows - 1) as f64,
            radius: RADIUS,
        };
        let circles = vec![
            top_left_circle_shape,
            top_right_circle_shape,
            bottom_left_circle_shape,
            bottom_right_circle_shape,
        ]
        .into_iter()
        .map(|shape| Component::Circle {
            fill_color: self.color(),
            shape: shape.translate(self.x(), self.y()),
            on_click: None,
        });

        rects.chain(circles).collect()
    }
}

const RADIUS: f64 = 110.0;
