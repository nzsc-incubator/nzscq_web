use super::Render;
use crate::paint::{Path, PathCommand, Stroke};
use crate::side::Side;

use crate::{
    colors,
    paint::{Component, ImageType},
    render::{
        lerp::{LerpInto, LerpableComponent, Lerper},
        switch::{Switch, Switch4},
    },
    shapes::{CenteredRect, Rect},
    transform::Translate,
};

pub struct ConstantHealthDisplay {
    pub side: Side,
    pub health: u8,
}

impl Render for ConstantHealthDisplay {
    fn render(&self) -> Vec<Component> {
        let hearts = (0..self.health as usize)
            .map(|i| {
                heart_from_position(HeartPosition {
                    side: self.side,
                    index: i,
                })
                .case(0.0)
                .expect("should find a case")
            })
            .flatten();
        let trapezoid = HealthTrapezoid { side: self.side };

        trapezoid.render().into_iter().chain(hearts).collect()
    }
}

pub struct FadingHealthDisplay {
    pub side: Side,
    pub starting_health: u8,
}

impl LerpInto<Vec<Component>> for FadingHealthDisplay {
    fn lerp_into(self, lerper: &Lerper) -> Vec<Component> {
        let sublerper = lerper.sub_lerper(0.0..colors::PORTION_OF_DURATION_SPENT_POPPING);
        let hearts = (0..self.starting_health)
            .map(|i| {
                let completion_factor = if i == self.starting_health - 1 {
                    sublerper.lerp(0.0, 1.0)
                } else {
                    0.0
                };
                heart_from_position(HeartPosition {
                    side: self.side,
                    index: i as usize,
                })
                .case(completion_factor)
                .expect("should find a case")
            })
            .flatten();
        let trapezoid = HealthTrapezoid { side: self.side };

        trapezoid.render().into_iter().chain(hearts).collect()
    }
}

const LEFT_0_CENTER: (f64, f64) = (80.0, 50.0);
const RIGHT_0_CENTER: (f64, f64) = (1720.0, 50.0);
const SIZE: f64 = 80.0;

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
    match position.side {
        Side::Left => Into::<Rect>::into(CenteredRect {
            x: LEFT_0_CENTER.0,
            y: LEFT_0_CENTER.1,
            width: scale * SIZE,
            height: scale * SIZE,
        })
        .translate(position.index as f64 * SIZE, 0.0),

        Side::Right => Into::<Rect>::into(CenteredRect {
            x: RIGHT_0_CENTER.0,
            y: RIGHT_0_CENTER.1,
            width: scale * SIZE,
            height: scale * SIZE,
        })
        .translate(position.index as f64 * -SIZE, 0.0),
    }
}

#[derive(Debug, Clone, Copy)]
struct HeartPosition {
    side: Side,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
struct HealthTrapezoid {
    side: Side,
}

impl Render for HealthTrapezoid {
    fn render(&self) -> Vec<Component> {
        let dx = match self.side {
            Side::Left => 20.0,
            Side::Right => 1340.0,
        };

        vec![Component::UnclickablePath {
            path: Path {
                start: (80.0, 0.0),
                commands: vec![
                    PathCommand::ArcTo(0.0, 0.0, 30.0, 70.0, 3.0),
                    PathCommand::ArcTo(40.0, 75.0, 415.0, 70.0, 8.0),
                    PathCommand::ArcTo(400.0, 75.0, 435.0, 0.0, 8.0),
                    PathCommand::ArcTo(440.0, 0.0, 435.0, 0.0, 3.0),
                ],
            },
            fill_color: Some(colors::TRAPEZOID_FILL),
            stroke: Some(Stroke {
                color: colors::TRAPEZOID_OUTCOME_SCREEN_BORDER,
                width: colors::TRAPEZOID_BORDER_WIDTH,
            }),
        }
        .translate(dx, 15.0)]
    }
}
