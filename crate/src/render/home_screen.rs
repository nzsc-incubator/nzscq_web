use crate::click::Action;
use crate::colors;
use crate::paint::{Component, ImageType};
use crate::shapes::Rect;

pub fn home_screen() -> Vec<Component> {
    vec![
        Component::Background {
            color: colors::HOME_SCREEN_BACKGROUND,
        },
        Component::Image {
            image_type: ImageType::Homescreen,
            alpha: 1.0,
            shape: Rect {
                x: 0.0,
                y: 0.0,
                width: 1800.0,
                height: 1000.0,
            },
            on_click: None,
        },
        Component::Image {
            image_type: ImageType::SinglePlayerButton,
            alpha: 1.0,
            shape: Rect {
                x: 706.0,
                y: 440.0,
                width: 388.0,
                height: 240.0,
            },
            on_click: Some(Action::StartSinglePlayerGame),
        },
        Component::Image {
            image_type: ImageType::MultiPlayerButton,
            alpha: 0.5,
            shape: Rect {
                x: 706.0,
                y: 720.0,
                width: 388.0,
                height: 240.0,
            },
            on_click: None,
        },
        Component::Image {
            image_type: ImageType::SettingsButton,
            alpha: 1.0,
            shape: Rect {
                x: 30.0,
                y: 30.0,
                width: 80.0,
                height: 80.0,
            },
            on_click: Some(Action::NavigateToSettingsScreen),
        },
    ]
}
