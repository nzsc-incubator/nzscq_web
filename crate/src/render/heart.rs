use crate::{
    paint::ImageType,
    render::{
        lerp::{LerpableComponent, Lerper},
        switch::{Switch, Switch4},
    },
    shapes::{CenteredRect, Rect, Translate},
};

const LEFT_0_CENTER: (f64, f64) = (80.0, 50.0);
const RIGHT_0_CENTER: (f64, f64) = (1720.0, 50.0);
const SIZE: f64 = 80.0;

pub fn left_at(index: usize) -> impl Switch {
    heart_from_position(HeartPosition::FromLeft(index))
}

pub fn right_at(index: usize) -> impl Switch {
    heart_from_position(HeartPosition::FromRight(index))
}

fn heart_from_position(position: HeartPosition) -> impl Switch {
    Switch4(
        (0.00..0.20, move |lerper: Lerper| {
            vec![lerper.lerp1(LerpableComponent::Image {
                image_type: ImageType::Heart,
                start_alpha: 1.0,
                end_alpha: 1.0,
                start_shape: scale_heart(&position, 1.0),
                end_shape: scale_heart(&position, 1.3),
                on_click: None,
            })]
        }),
        (0.20..0.35, move |lerper| {
            vec![lerper.lerp1(LerpableComponent::Image {
                image_type: ImageType::Heart,
                start_alpha: 1.0,
                end_alpha: 1.0,
                start_shape: scale_heart(&position, 1.3),
                end_shape: scale_heart(&position, 1.4),
                on_click: None,
            })]
        }),
        (0.35..0.50, move |lerper| {
            vec![lerper.lerp1(LerpableComponent::Image {
                image_type: ImageType::Heart,
                start_alpha: 1.0,
                end_alpha: 0.8,
                start_shape: scale_heart(&position, 1.4),
                end_shape: scale_heart(&position, 0.9),
                on_click: None,
            })]
        }),
        (0.50..=1.00, move |lerper| {
            vec![lerper.lerp1(LerpableComponent::Image {
                image_type: ImageType::Heart,
                start_alpha: 0.8,
                end_alpha: 0.0,
                start_shape: scale_heart(&position, 0.5),
                end_shape: scale_heart(&position, 2.0),
                on_click: None,
            })]
        }),
    )
}

fn scale_heart(position: &HeartPosition, scale: f64) -> Rect {
    match position {
        HeartPosition::FromLeft(index) => Into::<Rect>::into(CenteredRect {
            x: LEFT_0_CENTER.0,
            y: LEFT_0_CENTER.1,
            width: scale * SIZE,
            height: scale * SIZE,
        })
        .translate(*index as f64 * SIZE, 0.0),

        HeartPosition::FromRight(index) => Into::<Rect>::into(CenteredRect {
            x: RIGHT_0_CENTER.0,
            y: RIGHT_0_CENTER.1,
            width: scale * SIZE,
            height: scale * SIZE,
        })
        .translate(*index as f64 * -SIZE, 0.0),
    }
}

#[derive(Debug, Clone, Copy)]
enum HeartPosition {
    FromLeft(usize),
    FromRight(usize),
}
