use crate::helpers;
use crate::opponent::Difficulty;
use crate::paint::ImageType;

use nzscq::choices::{Booster, Character, Move};

use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

use std::collections::HashMap;
use std::convert::TryFrom;

pub struct ImageMap(HashMap<ImageType, HtmlImageElement>);

impl ImageMap {
    pub fn get(&self, key: ImageType) -> Option<&HtmlImageElement> {
        self.0.get(&key)
    }
}

impl TryFrom<Function> for ImageMap {
    type Error = JsValue;

    fn try_from(get_move_images: Function) -> Result<ImageMap, JsValue> {
        ImageMapBuilder::new(get_move_images).build()
    }
}

struct ImageMapBuilder {
    map: HashMap<ImageType, HtmlImageElement>,
    get_image: Function,
}

impl ImageMapBuilder {
    fn new(get_image: Function) -> ImageMapBuilder {
        ImageMapBuilder {
            map: HashMap::new(),
            get_image,
        }
    }

    fn build(mut self) -> Result<ImageMap, JsValue> {
        for m in Move::all() {
            self.insert(ImageType::Move(m), m.to_string())?;
        }

        for b in Booster::all() {
            self.insert(ImageType::Booster(b), helpers::booster_logo_move_string(b))?;
        }

        for c in Character::all() {
            self.insert(
                ImageType::Character(c),
                helpers::character_logo_move(c).to_string(),
            )?;
        }

        self.insert(ImageType::Heart, "Heart")?;
        self.insert(ImageType::Mirror, "Mirror")?;
        self.insert(ImageType::DeclineDequeue, "NoBooster")?;

        self.insert(ImageType::Homescreen, "Homescreen")?;
        self.insert(ImageType::SinglePlayerButton, "SinglePlayerButton")?;
        self.insert(ImageType::MultiPlayerButton, "MultiPlayerButton")?;
        self.insert(ImageType::SettingsButton, "SettingsButton")?;
        self.insert(ImageType::Star, "Star")?;
        self.insert(ImageType::EmptyStar, "EmptyStar")?;
        self.insert(
            ImageType::ComputerDifficulty(Difficulty::Stupid),
            "ComputerDifficultyStupid",
        )?;
        self.insert(
            ImageType::ComputerDifficulty(Difficulty::Easy),
            "ComputerDifficultyEasy",
        )?;
        self.insert(
            ImageType::ComputerDifficulty(Difficulty::Medium),
            "ComputerDifficultyMedium",
        )?;

        Ok(ImageMap(self.map))
    }

    fn insert<T: AsRef<str>>(&mut self, key: ImageType, argument: T) -> Result<(), JsValue> {
        let image = self
            .get_image
            .call1(&JsValue::NULL, &JsValue::from_str(argument.as_ref()))?;
        self.map.insert(key, image.dyn_into::<HtmlImageElement>()?);

        Ok(())
    }
}
