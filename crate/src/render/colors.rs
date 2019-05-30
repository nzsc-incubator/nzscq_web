use crate::helpers::{booster_logo_move, character_logo_move};

use nzscq::choices::{Booster, Character, Move};

pub const BACKGROUND: &str = "#F1F1F1";

pub fn character_color(c: &Character) -> &'static str {
    move_color(&character_logo_move(c))
}

pub fn booster_color(b: &Booster) -> &'static str {
    if b == &Booster::None {
        NO_BOOSTER_BACKGROUND
    } else {
        move_color(&booster_logo_move(b).unwrap())
    }
}

const NO_BOOSTER_BACKGROUND: &str = "#111111";

pub fn move_color(m: &Move) -> &'static str {
    match m {
        Move::Kick
        | Move::Nunchucks
        | Move::ShadowFireball
        | Move::RunInCircles
        | Move::LightningFastKarateChop
        | Move::Rampage
        | Move::Muscle
        | Move::Zap
        | Move::Gravedigger
        | Move::ZombieCorps
        | Move::Apocalypse
        | Move::Helmet
        | Move::Smash
        | Move::StrongSmash
        | Move::Lightning
        | Move::Earthquake
        | Move::Nose
        | Move::NoseOfTheTaunted => "#111111",

        Move::NinjaSword
        | Move::ShadowSlip
        | Move::Regenerate
        | Move::SamuraiSword
        | Move::Twist
        | Move::Bend
        | Move::AcidSpray
        | Move::MustacheMash
        | Move::BigHairyDeal => "#DDDDDD",

        Move::BackwardsMoustachio | Move::JugglingKnives => "#888888",
    }
}
