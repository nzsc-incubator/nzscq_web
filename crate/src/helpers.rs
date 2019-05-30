use crate::paint::{ImageMap, ImageType};

use js_sys::Function;
use nzscq::choices::{BatchChoices, Character, Move};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlImageElement};

use std::collections::HashMap;

pub fn log<T: std::fmt::Debug>(message: &T) {
    let message = format!("{:?}", message);
    let message = JsValue::from_str(&message[..]);
    console::log_1(&message);
}

pub fn all_moves() -> Vec<Move> {
    vec![
        Move::Kick,
        Move::NinjaSword,
        Move::Nunchucks,
        Move::ShadowFireball,
        Move::ShadowSlip,
        Move::RunInCircles,
        Move::LightningFastKarateChop,
        Move::Rampage,
        Move::Muscle,
        Move::Zap,
        Move::Regenerate,
        Move::Gravedigger,
        Move::ZombieCorps,
        Move::Apocalypse,
        Move::SamuraiSword,
        Move::Helmet,
        Move::Smash,
        Move::StrongSmash,
        Move::Lightning,
        Move::Earthquake,
        Move::Twist,
        Move::Bend,
        Move::JugglingKnives,
        Move::AcidSpray,
        Move::Nose,
        Move::BackwardsMoustachio,
        Move::NoseOfTheTaunted,
        Move::MustacheMash,
        Move::BigHairyDeal,
    ]
}

pub fn millis_to_secs(millis: f64) -> f64 {
    millis * 0.001
}

pub fn image_map_from_function(get_move_images: Function) -> Result<ImageMap, JsValue> {
    let mut map = HashMap::new();

    for m in all_moves() {
        let image =
            get_move_images.call1(&JsValue::NULL, &JsValue::from_str(&m.to_string()[..]))?;
        map.insert(ImageType::Move(m), image.dyn_into::<HtmlImageElement>()?);
    }

    Ok(map)
}

pub trait IntoConcreteBatchChoices<T> {
    fn into_concrete(self) -> Option<Vec<Vec<T>>>;
}

impl IntoConcreteBatchChoices<Character> for BatchChoices {
    fn into_concrete(self) -> Option<Vec<Vec<Character>>> {
        if let BatchChoices::Character(choices) = self {
            Some(choices)
        } else {
            None
        }
    }
}
