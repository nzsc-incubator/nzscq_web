use crate::{
    colors,
    paint::{Component, ImageType},
    render::{
        lerp::{LerpInto, LerpableComponent, Lerper},
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

pub struct ConstantHealthDisplay {
    pub human_health: u8,
    pub computer_health: u8,
}

impl Into<Vec<Component>> for ConstantHealthDisplay {
    fn into(self) -> Vec<Component> {
        let trapezoids = vec![
            Component::HealthTrapezoid {
                x: 20.0,
                y: 15.0,
                border_width: colors::TRAPEZOID_BORDER_WIDTH,
                border_color: colors::TRAPEZOID_OUTCOME_SCREEN_BORDER,
                fill_color: colors::TRAPEZOID_FILL,
            },
            Component::HealthTrapezoid {
                x: 1340.0,
                y: 15.0,
                border_width: colors::TRAPEZOID_BORDER_WIDTH,
                border_color: colors::TRAPEZOID_OUTCOME_SCREEN_BORDER,
                fill_color: colors::TRAPEZOID_FILL,
            },
        ];
        let human_hearts: Vec<Component> = (0..self.human_health as usize)
            .into_iter()
            .map(|i| left_at(i).case(0.0).expect("should find a case"))
            .flatten()
            .collect();
        let computer_hearts: Vec<Component> = (0..self.computer_health as usize)
            .into_iter()
            .map(|i| right_at(i).case(0.0).expect("should find a case"))
            .flatten()
            .collect();

        vec![trapezoids, human_hearts, computer_hearts]
            .into_iter()
            .flatten()
            .collect()
    }
}

pub struct FadingHealthDisplay {
    pub previous_human_health: u8,
    pub previous_computer_health: u8,
    pub is_human_losing_a_heart: bool,
    pub is_computer_losing_a_heart: bool,
}

impl LerpInto<Vec<Component>> for FadingHealthDisplay {
    fn lerp_into(self, lerper: &Lerper) -> Vec<Component> {
        let trapezoids = vec![
                Component::HealthTrapezoid {
                    x: 20.0,
                    y: 15.0,
                    border_width: colors::TRAPEZOID_BORDER_WIDTH,
                    border_color: colors::TRAPEZOID_OUTCOME_SCREEN_BORDER,
                    fill_color: colors::TRAPEZOID_FILL,
                },
                Component::HealthTrapezoid {
                    x: 1340.0,
                    y: 15.0,
                    border_width: colors::TRAPEZOID_BORDER_WIDTH,
                    border_color: colors::TRAPEZOID_OUTCOME_SCREEN_BORDER,
                    fill_color: colors::TRAPEZOID_FILL,
                },
            ];
            let human_hearts: Vec<Component> = (0..self.previous_human_health)
                .into_iter()
                .map(|i| {
                    let completion_factor = if i == self.previous_human_health - 1 && self.is_human_losing_a_heart {
                        lerper.lerp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    left_at(i as usize)
                        .case(completion_factor)
                        .expect("should find a case")
                })
                .flatten()
                .collect();
            let computer_hearts: Vec<Component> = (0..self.previous_computer_health)
                .into_iter()
                .map(|i| {
                    let completion_factor = if i == self.previous_computer_health - 1 && self.is_computer_losing_a_heart {
                        lerper.lerp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    right_at(i as usize)
                        .case(completion_factor)
                        .expect("should find a case")
                })
                .flatten()
                .collect();

        vec![trapezoids, human_hearts, computer_hearts]
            .into_iter()
            .flatten()
            .collect()
    }
}