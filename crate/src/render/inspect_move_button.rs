use crate::canvas_dimensions;
use crate::click::Action;
use crate::paint::{Component, ImageType};
use crate::render::Render;
use crate::shapes::{dequeue_circle, Rect};

pub struct InspectMoveButton {
    pub enabled: bool,
}

impl InspectMoveButton {
    pub const Y: f64 = dequeue_circle::ROW_0_Y - 0.5 * Self::HEIGHT;
    const WIDTH: f64 = 291.0;
    const HEIGHT: f64 = 180.0;
}

impl Render<()> for InspectMoveButton {
    fn render(&self, _: ()) -> Vec<Component> {
        let on_click = if self.enabled {
            Some(Action::WaitForUserToChooseMoveToInspect)
        } else {
            None
        };

        vec![Component::Image {
            image_type: ImageType::InspectMoveButton,
            alpha: 1.0,
            shape: Rect {
                x: canvas_dimensions::CENTER_X - 0.5 * Self::WIDTH,
                y: Self::Y,
                width: Self::WIDTH,
                height: Self::HEIGHT,
            },
            on_click,
        }]
    }
}
