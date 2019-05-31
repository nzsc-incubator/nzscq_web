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
    has_drawn_past_completion: bool,
}

impl App {
    const IDEAL_DIMENSIONS: (u32, u32) = (1800, 1000);
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
            has_drawn_past_completion: false,
        };

        app.init()?;

        Ok(app)
    }

    fn init(&mut self) -> Result<(), JsValue> {
        self.body.append_child(&self.canvas)?;
        self.init_canvas()?;

        Ok(())
    }

    fn init_canvas(&mut self) -> Result<(), JsValue> {
        let (ideal_width, ideal_height) = self.ideal_dimensions();
        self.canvas.set_width(ideal_width);
        self.canvas.set_height(ideal_height);
        self.canvas.style().set_property("position", "absolute")?;

        self.resize()
    }

    pub fn resize(&mut self) -> Result<(), JsValue> {
        let (actual_width, actual_height) = self.dimensions()?;
        let (actual_width, actual_height) = (actual_width as f64, actual_height as f64);
        let (ideal_width, ideal_height) = self.ideal_dimensions();
        let (ideal_width, ideal_height) = (ideal_width as f64, ideal_height as f64);

        let style = self.canvas.style();

        if self.aspect()? > self.ideal_aspect() {
            let scale = actual_height / ideal_height;
            let scaled_width = ideal_width * scale;
            let left = (actual_width - scaled_width) / 2.0;

            style.set_property("height", &helpers::px(actual_height)[..])?;
            style.set_property("width", &helpers::px(scaled_width)[..])?;
            style.set_property("left", &helpers::px(left)[..])?;

            style.set_property("top", "0")?;
        } else {
            let scale = actual_width / ideal_width;
            let scaled_height = ideal_height * scale;
            let top = (actual_height - scaled_height) / 2.0;
            style.set_property("width", &helpers::px(actual_width)[..])?;
            style.set_property("height", &helpers::px(scaled_height)[..])?;
            style.set_property("top", &helpers::px(top)[..])?;

            style.set_property("left", "0")?;
        }

        self.draw()
    }

    fn aspect(&self) -> Result<f64, JsValue> {
        let (width, height) = self.dimensions()?;

        Ok(width as f64 / height as f64)
    }

    fn ideal_aspect(&self) -> f64 {
        let (width, height) = self.ideal_dimensions();

        width as f64 / height as f64
    }

    pub fn draw_if_needed(&mut self) -> Result<(), JsValue> {
        if self.completion_factor() < 1.0 {
            self.draw()
        } else {
            if self.has_drawn_past_completion {
                Ok(())
            } else {
                self.has_drawn_past_completion = true;
                self.draw()
            }
        }
    }

    fn draw(&mut self) -> Result<(), JsValue> {
        let components = (self.completion_factor(), &self.phase).render();
        let ideal_dimensions = self.ideal_dimensions();
        let body_style = self.body.style();
        let mut painter = Painter::new(&self.ctx, &body_style, &self.image_map, ideal_dimensions);
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
