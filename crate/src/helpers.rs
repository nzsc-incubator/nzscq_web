use crate::paint::{ImageMap, ImageType};

use js_sys::Function;
use nzscq::choices::{BatchChoices, Booster, Character, Move};
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

    for m in all_moves() {
        let image =
            get_move_images.call1(&JsValue::NULL, &JsValue::from_str(&m.to_string()[..]))?;
        map.insert(ImageType::Move(m), image.dyn_into::<HtmlImageElement>()?);
    }

    for b in all_boosters() {
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

fn all_moves() -> Vec<Move> {
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

fn all_boosters() -> Vec<Booster> {
    vec![
        Booster::Shadow,
        Booster::Speedy,
        Booster::Regenerative,
        Booster::ZombieCorps,
        Booster::Atlas,
        Booster::Strong,
        Booster::Backwards,
        Booster::Moustachio,
        Booster::None,
    ]
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
