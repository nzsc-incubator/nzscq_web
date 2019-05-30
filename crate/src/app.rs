use crate::{
    helpers::{self, IntoConcreteBatchChoices},
    paint::{ImageMap, Painter},
    phase::Phase,
    render::Render,
};

use js_sys::{Date, Function};
use nzscq::{choices::Character, game::BatchChoiceGame};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, Window};

#[wasm_bindgen]
pub struct App {
    window: Window,
    document: Document,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    image_map: ImageMap,
    game: BatchChoiceGame,
    phase: Phase,
    animation_start_secs: f64,
}

impl App {
    const IDEAL_DIMENSIONS: (u32, u32) = (900, 500);
    const HUMAN: usize = 0;
}

#[wasm_bindgen]
impl App {
    pub fn new(get_image: Function) -> Result<App, JsValue> {
        let window = web_sys::window().expect("should have a Window");
        let document = window.document().expect("should have a Document");
        let body = document.body().expect("should have a body");
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        let image_map = helpers::image_map_from_function(get_image)?;
        let game = BatchChoiceGame::default();
        let initial_human_choices = game.choices().into_concrete().unwrap().remove(App::HUMAN);

        let mut app = App {
            window,
            document,
            body,
            canvas,
            ctx,
            image_map,
            game,
            phase: Phase::ChoosingCharacters {
                previously_available: Character::all(),
                currently_available: initial_human_choices,
            },
            animation_start_secs: helpers::millis_to_secs(Date::now()),
        };

        app.init()?;

        Ok(app)
    }

    fn init(&mut self) -> Result<(), JsValue> {
        self.body.append_child(&self.canvas)?;
        self.resize()?;

        Ok(())
    }

    pub fn resize(&mut self) -> Result<(), JsValue> {
        let (width, height) = self.dimensions()?;
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.draw()?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), JsValue> {
        let components = (self.completion_factor(), &self.phase).render();
        let ideal_dimensions = self.ideal_dimensions();
        let dimensions = self.dimensions()?;
        let mut painter =
            Painter::new(&mut self.ctx, &self.image_map, ideal_dimensions, dimensions);
        painter.paint(components)?;

        Ok(())
    }

    fn completion_factor(&self) -> f64 {
        let current_time = helpers::millis_to_secs(Date::now());
        let time_after_start = current_time - self.animation_start_secs;
        let completion_factor = time_after_start / self.phase.duration_secs();
        let completion_factor = completion_factor.min(1.0);

        completion_factor
    }

    fn ideal_dimensions(&self) -> (u32, u32) {
        App::IDEAL_DIMENSIONS
    }

    fn dimensions(&self) -> Result<(u32, u32), JsValue> {
        let width = self.window.inner_width()?.as_f64().unwrap() as u32;
        let height = self.window.inner_height()?.as_f64().unwrap() as u32;

        Ok((width, height))
    }
}
