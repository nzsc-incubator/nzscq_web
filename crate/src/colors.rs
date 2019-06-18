use crate::helpers;

use nzscq::choices::{Booster, Character, Move, ArsenalItem};

pub const BACKGROUND: Rgba = Rgba(0xF1, 0xF1, 0xF1, 0xFF);
pub const OVERLAY: Rgba = Rgba(0x33, 0x33, 0x33, 0xAA);
pub const PORTION_OF_DURATION_SPENT_FADING: f64 = 1.0 / (5.0 * 0.55);
pub const PORTION_OF_DURATION_SPENT_POPPING: f64 = 0.6 / 1.1;
pub const DISABLED_DEQUEUE_ARSENAL_ITEM_ALPHA: u8 = 0x80;
// TODO Replace red placeholder with actual color
pub const DECLINE_DEQUEUE_COLOR: Rgba = Rgba(0xFF, 0x00, 0x00, 0xFF);

pub const TRAPEZOID_BORDER_WIDTH: f64 = 4.0;
pub const TRAPEZOID_BORDER: Rgba = Rgba(0x77, 0x77, 0x77, 0xFF);
pub const TRAPEZOID_OUTCOME_SCREEN_BORDER: Rgba = Rgba(0x49, 0x49, 0x49, 0xFF);
pub const TRAPEZOID_FILL: Rgba = BACKGROUND;

pub fn character_color(c: Character) -> Rgba {
    move_color(helpers::character_logo_move(c))
}

pub fn booster_color(b: Booster) -> Rgba {
    if b == Booster::None {
        NO_BOOSTER_BACKGROUND
    } else {
        move_color(helpers::booster_logo_move(b).unwrap())
    }
}

const NO_BOOSTER_BACKGROUND: Rgba = Rgba(0x11, 0x11, 0x11, 0xFF);

pub fn move_color(m: Move) -> Rgba {
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
        | Move::NoseOfTheTaunted => Rgba::opaque(0x11, 0x11, 0x11),

        Move::NinjaSword
        | Move::ShadowSlip
        | Move::Regenerate
        | Move::SamuraiSword
        | Move::Twist
        | Move::Bend
        | Move::AcidSpray
        | Move::MustacheMash
        | Move::BigHairyDeal => Rgba::opaque(0xDD, 0xDD, 0xDD),

        Move::BackwardsMoustachio | Move::JugglingKnives => Rgba::opaque(0x88, 0x88, 0x88),
    }
}

pub fn arsenal_item_color(arsenal_item: ArsenalItem) -> Rgba {
    match arsenal_item {
        ArsenalItem::Move(move_) => move_color(move_),
        ArsenalItem::Mirror => MIRROR_COLOR
    }
}

// TODO Replace red placeholder
const MIRROR_COLOR: Rgba = Rgba(0xFF, 0x00, 0x00, 0xFF);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub fn opaque(r: u8, g: u8, b: u8) -> Rgba {
        Rgba(r, g, b, 0xFF)
    }

    pub fn transparent() -> Rgba {
        Rgba(0, 0, 0, 0)
    }

    pub fn composite(colors: Vec<Rgba>) -> Rgba {
        colors
            .into_iter()
            .fold(Rgba::transparent(), |acc, color| color.composite_over(acc))
    }

    pub fn to_upper_hash_hex(&self) -> String {
        let Rgba(r, g, b, a) = self;

        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }

    pub fn with_alpha(self, alpha: u8) -> Rgba {
        Rgba(self.0, self.1, self.2, alpha)
    }

    pub fn composite_over(self, background: Rgba) -> Rgba {
        let (fg_color, fg_alpha) = self.into();
        let (bg_color, bg_alpha) = background.into();
        let out_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);

        Rgba::from((
            (fg_color * fg_alpha + bg_color * bg_alpha * (1.0 - fg_alpha)) / out_alpha,
            out_alpha,
        ))
    }
}

impl Into<(Rgb, f64)> for Rgba {
    fn into(self) -> (Rgb, f64) {
        let color = Rgb(self.0 as f64, self.1 as f64, self.2 as f64);
        let alpha = self.3 as f64 / 255.0;

        (color, alpha)
    }
}

impl From<(Rgb, f64)> for Rgba {
    fn from((rgb, a): (Rgb, f64)) -> Rgba {
        Rgba(rgb.0 as u8, rgb.1 as u8, rgb.2 as u8, (a * 255.0) as u8)
    }
}

struct Rgb(pub f64, pub f64, pub f64);

impl std::ops::Mul<f64> for Rgb {
    type Output = Rgb;

    fn mul(self, alpha: f64) -> Rgb {
        Rgb(self.0 * alpha, self.1 * alpha, self.2 * alpha)
    }
}

impl std::ops::Div<f64> for Rgb {
    type Output = Rgb;

    fn div(self, alpha: f64) -> Rgb {
        Rgb(self.0 / alpha, self.1 / alpha, self.2 / alpha)
    }
}

impl std::ops::Add for Rgb {
    type Output = Rgb;

    fn add(self, other: Rgb) -> Rgb {
        Rgb(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
