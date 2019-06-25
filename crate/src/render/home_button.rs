use crate::canvas_dimensions;
use crate::click::Action;
use crate::colors;
use crate::paint::{Component, Path, PathCommand};
use crate::render::lerp::Lerper;
use crate::shapes::{Circle, Rect, Translate};

pub fn home_button(lerper: &Lerper) -> Vec<Component> {
    vec![
        Component::Circle {
            fill_color: colors::HOME_BUTTON_BACKGROUND,
            shape: Circle {
                x: CENTER_X,
                y: CENTER_Y,
                radius: BACKGROUND_RADIUS,
            },
            on_click: Some(Action::NavigateHome),
        },
        Component::UnclickablePath {
            path: Path {
                start: (900.0, 420.0),
                commands: vec![
                    PathCommand::LineTo(820.0, 490.0),
                    PathCommand::LineTo(850.0, 490.0),
                    PathCommand::LineTo(850.0, 560.0),
                    PathCommand::LineTo(950.0, 560.0),
                    PathCommand::LineTo(950.0, 490.0),
                    PathCommand::LineTo(980.0, 490.0),
                ],
            },
            fill_color: Some(colors::HOME_BUTTON_FOREGROUND),
            stroke: None,
        },
        Component::Circle {
            fill_color: colors::HOME_BUTTON_BACKGROUND,
            shape: Circle {
                x: 900.0,
                y: 520.0,
                radius: 15.0,
            },
            on_click: None,
        },
        Component::Rect {
            fill_color: colors::HOME_BUTTON_BACKGROUND,
            shape: Rect {
                x: 885.0,
                y: 520.0,
                width: 30.0,
                height: 40.0,
            },
            on_click: None,
        },
    ]
    .translate(0.0, lerper.lerp(BACKGROUND_RADIUS + CENTER_Y, 0.0))
}

const BACKGROUND_RADIUS: f64 = 120.0;
const CENTER_X: f64 = 0.5 * canvas_dimensions::WIDTH;
const CENTER_Y: f64 = 0.5 * canvas_dimensions::HEIGHT;
