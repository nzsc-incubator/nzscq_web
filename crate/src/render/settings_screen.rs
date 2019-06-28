use crate::canvas_dimensions;
use crate::colors;
use crate::paint::{Component, ImageType};
use crate::render::{self, lerp::Lerper};
use crate::transform::{Scale, Translate};

pub fn settings_screen() -> Vec<Component> {
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
    ]
    .into_iter()
    .flatten()
    .collect()
}

const ORIGINAL_RADIUS: f64 = 120.0;
const MARGIN: f64 = 30.0;
const TARGET_RADIUS: f64 = 40.0;