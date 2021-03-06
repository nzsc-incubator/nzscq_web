use crate::{
    click,
    context::Context,
    helpers,
    image_map::ImageMap,
    letterbox::Letterbox,
    opponent::{Difficulty, Random},
    paint::{Component, Painter},
    render::Render,
    state::State,
};

use js_sys::{Date, Function, Math};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, Window};

use std::convert::TryInto;
use std::f64;
use std::string::ToString;

#[wasm_bindgen]
pub struct App {
    window: Window,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    image_map: ImageMap,
    context: Context,
    state: State,
    has_drawn_past_completion: bool,
}

impl App {
    const IDEAL_DIMENSIONS: (u32, u32) = (1800, 1000);
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
        let computer_difficulty =
            helpers::get_local_storage_item(&window, "nzscq_computer_difficulty")
                .unwrap_or_else(|| "fail".to_string())
                .try_into()
                .unwrap_or(Difficulty::Medium);
        let current_time = helpers::millis_to_secs(Date::now());

        let mut app = App {
            window,
            body,
            canvas,
            ctx,
            image_map: get_image
                .try_into()
                .expect("should be able to create image map from js image getter"),
            context: Context {
                computer_difficulty,
                current_time,
            },
            state: State::HomeScreen,
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

    pub fn call_with_canvas(&self, callback: Function) -> Result<JsValue, JsValue> {
        callback.call1(&JsValue::NULL, &self.canvas)
    }

    pub fn resize(&mut self) -> Result<(), JsValue> {
        let (actual_width, actual_height) = self.dimensions()?;
        let (actual_width, actual_height) = (f64::from(actual_width), f64::from(actual_height));
        let (ideal_width, ideal_height) = self.ideal_dimensions();
        let (ideal_width, ideal_height) = (f64::from(ideal_width), f64::from(ideal_height));

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

        Ok(f64::from(width) / f64::from(height))
    }

    fn ideal_aspect(&self) -> f64 {
        let (width, height) = self.ideal_dimensions();

        f64::from(width) / f64::from(height)
    }

    pub fn on_click(&mut self, client_x: u32, client_y: u32) -> Result<(), JsValue> {
        let canvas_coords = self.canvas_coords((client_x, client_y))?;
        let components = self.render();
        let action = click::action_triggered_by_click_at(canvas_coords, &components);
        if let Some(action) = action {
            self.handle_action(action);
        }

        Ok(())
    }

    fn canvas_coords(&self, client_coords: (u32, u32)) -> Result<(f64, f64), JsValue> {
        let (x, y) = client_coords;
        let (mut x, mut y) = (f64::from(x), f64::from(y));
        let letterbox = self.letterbox()?;
        x -= letterbox.left;
        y -= letterbox.top;
        x /= letterbox.scale;
        y /= letterbox.scale;

        Ok((x, y))
    }

    fn letterbox(&self) -> Result<Letterbox, JsValue> {
        Ok(Letterbox::new(self.ideal_dimensions(), self.dimensions()?))
    }

    fn handle_action(&mut self, action: click::Action) {
        let mut should_start_animation = true;
        let mut prevent_animation_start = || should_start_animation = false;

        match &mut self.state {
            State::HomeScreen => match action {
                click::Action::StartSinglePlayerGame => {
                    self.state.start_single_player_game(
                        helpers::millis_to_secs(Date::now()),
                        &JsPrng.random().to_string()[..],
                        self.context.computer_difficulty,
                    );
                }

                click::Action::NavigateToSettingsScreen => self.state = State::SettingsScreen,

                action => panic!(
                    "Action {:?} should never be emitted when state == Homescreen",
                    action
                ),
            },

            State::SettingsScreen => match action {
                click::Action::NavigateHome => self.state = State::HomeScreen,
                click::Action::SetComputerDifficulty(difficulty) => {
                    self.context.computer_difficulty = difficulty;
                    helpers::set_local_storage_item(
                        &self.window,
                        "nzscq_computer_difficulty",
                        &difficulty.to_string()[..],
                    )
                }
                click::Action::PromptUserForCustomSeed => {
                    let seed = self
                        .window
                        .prompt_with_message("Enter your seed:")
                        .expect("should be able to prompt user for seed");
                    if let Some(seed) = seed {
                        self.state.start_single_player_game(
                            helpers::millis_to_secs(Date::now()),
                            &seed[..],
                            self.context.computer_difficulty,
                        );
                    }
                }

                action => panic!(
                    "Action {:?} should never be emitted when state == SettingsScreen",
                    action
                ),
            },

            State::SinglePlayer(state) => match action {
                click::Action::ChooseCharacter(human_character) => {
                    state.handle_character_choice(human_character)
                }

                click::Action::ChooseBooster(human_booster) => {
                    state.handle_booster_choice(human_booster)
                }

                click::Action::ChooseDequeue(human_dequeue) => {
                    state.handle_dequeue_choice(human_dequeue)
                }

                click::Action::ChooseAction(human_action) => {
                    state.handle_action_choice(human_action)
                }

                click::Action::NavigateHome => {
                    self.state = State::HomeScreen;
                }

                click::Action::WaitForUserToChooseMoveToInspect => {
                    prevent_animation_start();
                    state
                        .phase
                        .wait_for_user_to_choose_move_to_inspect()
                        .expect("should be able to wait for user to choose move to inspect");
                }

                click::Action::InspectMove(m) => {
                    prevent_animation_start();
                    state
                        .phase
                        .inspect_move(m)
                        .expect("should be able to inspect move")
                }

                click::Action::StopInspectingMove => {
                    prevent_animation_start();
                    state
                        .phase
                        .stop_inspecting_move()
                        .expect("should be able to stop inspecting move");
                }

                action => panic!(
                    "Action {:?} should never be emitted when state == SinglePlayer",
                    action
                ),
            },
        }

        if should_start_animation {
            self.start_animation();
        } else {
            self.has_drawn_past_completion = false;
        }
    }

    fn start_animation(&mut self) {
        self.has_drawn_past_completion = false;
        self.state
            .start_animation(helpers::millis_to_secs(Date::now()));
    }

    pub fn draw_if_needed(&mut self) -> Result<(), JsValue> {
        let current_time = helpers::millis_to_secs(Date::now());

        if self.state.is_current_time_past_completion(current_time) {
            if !self.has_drawn_past_completion {
                self.has_drawn_past_completion = true;

                self.draw()
            } else {
                Ok(())
            }
        } else {
            self.draw()
        }
    }

    fn draw(&mut self) -> Result<(), JsValue> {
        let components = self.render();
        let ideal_dimensions = self.ideal_dimensions();
        let body_style = self.body.style();
        let mut painter = Painter::new(&self.ctx, &body_style, &self.image_map, ideal_dimensions);
        painter.paint(components)?;

        Ok(())
    }

    fn render(&mut self) -> Vec<Component> {
        self.context.current_time = helpers::millis_to_secs(Date::now());

        self.state.render(&self.context)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsPrng;

impl Random for JsPrng {
    fn random(&mut self) -> f64 {
        Math::random()
    }
}
