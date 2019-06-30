use crate::click::Action;
use crate::colors;
use crate::paint::{Component, ImageType};
use crate::shapes::dequeue_circle::{self, CirclePosition};

use nzscq::choices::ArsenalItem;

pub fn arsenal_item_display(
    item: ArsenalItem,
    enabled: bool,
    on_click_if_enabled: Option<Action>,
    position: CirclePosition,
) -> Vec<Component> {
    let on_click = if enabled { on_click_if_enabled } else { None };
    let CirclePosition { side, column, row } = position;
    let side = side;
    let fill_color = if enabled {
        colors::arsenal_item_color(item)
    } else {
        colors::arsenal_item_color(item).with_alpha(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA)
    };
    let image_alpha = if enabled {
        1.0
    } else {
        f64::from(colors::DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA) / 255.0
    };

    vec![
        vec![
            Component::Circle {
                fill_color: fill_color,
                shape: dequeue_circle::background_at(side, row, column),
                on_click,
            },
            Component::Image {
                image_type: ImageType::from(item),
                alpha: image_alpha,
                shape: dequeue_circle::foreground_at(side, row, column),
                on_click: None,
            },
        ],
        if enabled || item == ArsenalItem::Mirror {
            vec![]
        } else {
            vec![Component::Circle {
                fill_color: colors::OVERLAY,
                shape: dequeue_circle::background_at(side, row, column),
                on_click: None,
            }]
        },
    ]
    .into_iter()
    .flatten()
    .collect()
}
