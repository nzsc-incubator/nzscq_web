use crate::canvas_dimensions;
use crate::click::Action;
use crate::colors;
use crate::context::Context;
use crate::opponent::Difficulty;
use crate::paint::{Component, ImageType};
use crate::render::{self, lerp::Lerper};
use crate::shapes::Rect;
use crate::transform::{Scale, Translate};

use std::convert::TryFrom;
use std::f64;

pub fn settings_screen(context: &Context) -> Vec<Component> {
    let difficulty = context.computer_difficulty;

    vec![
        vec![Component::Background {
            color: colors::SETTINGS_SCREEN_BACKGROUND,
        }],
        render::home_button(&Lerper::from_completion_factor(1.0))
            .translate(
                -0.5 * canvas_dimensions::WIDTH,
                -0.5 * canvas_dimensions::HEIGHT,
            )
            .scale(TARGET_RADIUS / ORIGINAL_RADIUS)
            .translate(MARGIN + TARGET_RADIUS, MARGIN + TARGET_RADIUS),
        vec![Component::Image {
            image_type: ImageType::ComputerDifficulty(difficulty),
            alpha: 1.0,
            shape: Rect {
                x: 160.0,
                y: -50.0,
                width: 776.0,
                height: 240.0,
            },
            on_click: None,
        }],
        (0..3)
            .map(|i| Component::Image {
                image_type: if i > difficulty as u8 {
                    ImageType::EmptyStar
                } else {
                    ImageType::Star
                },
                alpha: 1.0,
                shape: Rect {
                    x: (160.0 + 776.0) + 100.0 * f64::from(i),
                    y: 25.0,
                    width: 80.0,
                    height: 80.0,
                },
                on_click: Some(Action::SetComputerDifficulty(
                    Difficulty::try_from(i)
                        .expect("a u8 in 0..3 should be able to convert to a Difficulty"),
                )),
            })
            .collect(),
        vec![
            Component::Image {
                image_type: ImageType::TutorialButton,
                alpha: 0.5,
                shape: Rect {
                    x: 160.0,
                    y: 150.0,
                    width: 388.0,
                    height: 240.0,
                },
                on_click: None,
            },
            Component::Image {
                image_type: ImageType::PassAndPlayButton,
                alpha: 0.5,
                shape: Rect {
                    x: 160.0 + 1.0 * (388.0 + 40.0),
                    y: 150.0,
                    width: 388.0,
                    height: 240.0,
                },
                on_click: None,
            },
            Component::Image {
                image_type: ImageType::CustomSeedButton,
                alpha: 1.0,
                shape: Rect {
                    x: 160.0 + 2.0 * (388.0 + 40.0),
                    y: 150.0,
                    width: 388.0,
                    height: 240.0,
                },
                on_click: Some(Action::PromptUserForCustomSeed),
            },
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}

const ORIGINAL_RADIUS: f64 = 120.0;
const MARGIN: f64 = 30.0;
const TARGET_RADIUS: f64 = 40.0;
