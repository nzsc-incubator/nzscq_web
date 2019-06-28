use crate::{
    opponent::Difficulty,
    paint::Component,
    shapes::{Circle, Rect},
};

use nzscq::choices::{self, Booster, Character, DequeueChoice};

pub fn action_triggered_by_click_at(
    canvas_coords: (f64, f64),
    components: &Vec<Component>,
) -> Option<Action> {
    let front_to_back = components.iter().rev();

    for component in front_to_back {
        if component.touches(canvas_coords) {
            if let Some(on_click) = component.on_click() {
                return Some(on_click);
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
pub enum Action {
    StartSinglePlayerGame,
    NavigateToSettingsScreen,

    ChooseCharacter(Character),
    ChooseBooster(Booster),
    ChooseDequeue(DequeueChoice),
    ChooseAction(choices::Action),

    SetComputerDifficulty(Difficulty),
    NavigateHome,

    StopPropagation,
}

trait Touches {
    fn touches(&self, canvas_coords: (f64, f64)) -> bool;
}

impl Touches for Component {
    fn touches(&self, canvas_coords: (f64, f64)) -> bool {
        match self {
            Component::Background { .. } => true,
            Component::Rect { shape, .. } => shape.touches(canvas_coords),
            Component::Circle { shape, .. } => shape.touches(canvas_coords),
            Component::Image { shape, .. } => shape.touches(canvas_coords),
            Component::UnclickablePath { .. } => false,
        }
    }
}

impl Touches for Rect {
    fn touches(&self, canvas_coords: (f64, f64)) -> bool {
        let (x, y) = canvas_coords;
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

impl Touches for Circle {
    fn touches(&self, canvas_coords: (f64, f64)) -> bool {
        let (x, y) = canvas_coords;
        let (dx, dy) = (self.x - x, self.y - y);
        let distance_squared = dx * dx + dy * dy;
        let radius_squared = self.radius * self.radius;

        distance_squared <= radius_squared
    }
}
