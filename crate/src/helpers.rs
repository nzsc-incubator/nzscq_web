use nzscq::choices::{Booster, Character, Move};
use wasm_bindgen::JsValue;
use web_sys::console;

pub fn log<T: std::fmt::Debug>(message: &T) {
    let message = format!("{:?}", message);
    let message = JsValue::from_str(&message[..]);
    console::log_1(&message);
}

pub fn millis_to_secs(millis: f64) -> f64 {
    millis * 0.001
}

pub fn character_logo_move(c: Character) -> Move {
    match c {
        Character::Ninja => Move::Kick,
        Character::Zombie => Move::Rampage,
        Character::Samurai => Move::Helmet,
        Character::Clown => Move::Nose,
    }
}

pub fn booster_logo_move_string(b: Booster) -> String {
    let logo_move = booster_logo_move(b);

    logo_move
        .map(|m| m.to_string())
        .unwrap_or_else(|| "NoBooster".to_string())
}

pub fn booster_logo_move(b: Booster) -> Option<Move> {
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

pub fn opponent_points_to_own_health(points: u8) -> u8 {
    5 - points
}

pub fn vec2_to_arr2<T>(mut vec: Vec<T>) -> [T; 2] {
    [vec.remove(0), vec.remove(0)]
}

pub fn height_in_rows<T>(items: &[T], columns: usize) -> usize {
    (items.len() + columns - 1) / columns
}

pub const SQRT_3: f64 = 1.732_050_807_568_877_2;
