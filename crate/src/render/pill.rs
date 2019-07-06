use super::Render;
use crate::colors::{self, Rgba};
use crate::paint::Component;
use crate::shapes::{
    dequeue_circle::{self, CirclePosition},
    Circle, Rect,
};
use crate::side::Side;
use crate::transform::Translate;

pub struct Pill {
    pub position: CirclePosition,
    pub width_in_columns: usize,
    pub height_in_rows: usize,
    pub enabled: bool,
}

impl Pill {
    fn unchecked_render(&self) -> Vec<Component> {
        let vertical_connector = Rect {
            x: self.leftmost_circle_x(),
            y: -RADIUS,
            width: (self.raw_offset() * (self.width_in_columns - 1) as f64),
            height: (2.0 * RADIUS) + (self.raw_offset() * (self.height_in_rows - 1) as f64),
        };
        let horizontal_connector = Rect {
            x: self.leftmost_circle_x() - RADIUS,
            y: 0.0,
            width: (2.0 * RADIUS) + (self.raw_offset() * (self.width_in_columns - 1) as f64),
            height: if self.height_in_rows < 2 {
                0.0
            } else {
                (2.0 * RADIUS) + (self.raw_offset() * (self.height_in_rows - 2) as f64)
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
            x: self.adjusted_offset() * (self.width_in_columns - 1) as f64,
            y: 0.0,
            radius: RADIUS,
        };
        let bottom_left_circle_shape = Circle {
            x: 0.0,
            y: self.raw_offset() * (self.height_in_rows - 1) as f64,
            radius: RADIUS,
        };
        let bottom_right_circle_shape = Circle {
            x: self.adjusted_offset() * (self.width_in_columns - 1) as f64,
            y: self.raw_offset() * (self.height_in_rows - 1) as f64,
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

    fn adjusted_offset(&self) -> f64 {
        let factor = match self.position.side {
            Side::Left => 1.0,
            Side::Right => -1.0,
        };

        factor * self.raw_offset()
    }

    fn raw_offset(&self) -> f64 {
        dequeue_circle::OFFSET
    }

    fn leftmost_circle_x(&self) -> f64 {
        match self.position.side {
            Side::Left => 0.0,
            Side::Right => self.adjusted_offset() * (self.width_in_columns - 1) as f64,
        }
    }
}

impl Render<()> for Pill {
    fn render(&self, _: ()) -> Vec<Component> {
        if self.width_in_columns == 0 || self.height_in_rows == 0 {
            vec![]
        } else {
            self.unchecked_render()
        }
    }
}

const RADIUS: f64 = 110.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pill_with_zero_width_renders_empty_vec() {
        let pill = Pill {
            position: CirclePosition {
                side: Side::Left,
                column: 0,
                row: 0,
            },
            width_in_columns: 0,
            height_in_rows: 1,
            enabled: true,
        };
        assert!(pill.render(()).is_empty());
    }

    #[test]
    fn pill_with_zero_height_renders_empty_vec() {
        let pill = Pill {
            position: CirclePosition {
                side: Side::Left,
                column: 0,
                row: 0,
            },
            width_in_columns: 3,
            height_in_rows: 0,
            enabled: true,
        };
        assert!(pill.render(()).is_empty());
    }
}
