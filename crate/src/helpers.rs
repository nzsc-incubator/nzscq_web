use crate::paint::{ImageMap, ImageType};

use js_sys::Function;
use nzscq::choices::{Action, BatchChoices, Booster, Character, DequeueChoice, Move};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlImageElement};

use std::collections::HashMap;

pub fn log<T: std::fmt::Debug>(message: &T) {
    let message = format!("{:?}", message);
    let message = JsValue::from_str(&message[..]);
    console::log_1(&message);
}

pub fn millis_to_secs(millis: f64) -> f64 {
    millis * 0.001
}

pub fn image_map_from_function(get_move_images: Function) -> Result<ImageMap, JsValue> {
    let mut map = HashMap::new();

    for m in Move::all() {
        let image =
            get_move_images.call1(&JsValue::NULL, &JsValue::from_str(&m.to_string()[..]))?;
        map.insert(ImageType::Move(m), image.dyn_into::<HtmlImageElement>()?);
    }

    for b in Booster::all() {
        let image = if b == Booster::None {
            get_move_images.call1(&JsValue::NULL, &JsValue::from_str("No Booster"))
        } else {
            let logo_move = booster_logo_move(&b).unwrap();
            get_move_images.call1(
                &JsValue::NULL,
                &JsValue::from_str(&logo_move.to_string()[..]),
            )
        }?;
        map.insert(ImageType::Booster(b), image.dyn_into::<HtmlImageElement>()?);
    }

    for c in Character::all() {
        let logo_move = character_logo_move(&c);
        let image = get_move_images.call1(
            &JsValue::NULL,
            &JsValue::from_str(&logo_move.to_string()[..]),
        )?;
        map.insert(
            ImageType::Character(c),
            image.dyn_into::<HtmlImageElement>()?,
        );
    }

    Ok(map)
}

pub fn character_logo_move(c: &Character) -> Move {
    match c {
        Character::Ninja => Move::Kick,
        Character::Zombie => Move::Rampage,
        Character::Samurai => Move::Helmet,
        Character::Clown => Move::Nose,
    }
}

pub fn booster_logo_move(b: &Booster) -> Option<Move> {
    match b {
        Booster::Shadow => Some(Move::ShadowSlip),
        Booster::Speedy => Some(Move::LightningFastKarateChop),
        Booster::Regenerative => Some(Move::Regenerate),
        Booster::ZombieCorps => Some(Move::ZombieCorps),
        Booster::Atlas => Some(Move::Lightning),
        Booster::Strong => Some(Move::Bend),
        Booster::Backwards => Some(Move::BackwardsMoustachio),
        Booster::Moustachio => Some(Move::BigHairyDeal),
        Booster::None => None,
    }
}

pub fn px(px: f64) -> String {
    format!("{}px", px)
}

pub trait IntoConcreteBatchChoices<T> {
    fn into_concrete(self) -> Option<Vec<Vec<T>>>;
}

impl IntoConcreteBatchChoices<Character> for BatchChoices {
    fn into_concrete(self) -> Option<Vec<Vec<Character>>> {
        if let BatchChoices::Characters(choices) = self {
            Some(choices)
        } else {
            None
        }
    }
}

impl IntoConcreteBatchChoices<Booster> for BatchChoices {
    fn into_concrete(self) -> Option<Vec<Vec<Booster>>> {
        if let BatchChoices::Boosters(choices) = self {
            Some(choices)
        } else {
            None
        }
    }
}

impl IntoConcreteBatchChoices<DequeueChoice> for BatchChoices {
    fn into_concrete(self) -> Option<Vec<Vec<DequeueChoice>>> {
        if let BatchChoices::DequeueChoices(choices) = self {
            Some(choices)
        } else {
            None
        }
    }
}

impl IntoConcreteBatchChoices<Action> for BatchChoices {
    fn into_concrete(self) -> Option<Vec<Vec<Action>>> {
        if let BatchChoices::Actions(choices) = self {
            Some(choices)
        } else {
            None
        }
    }
}
