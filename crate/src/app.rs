use crate::{
    click, helpers,
    letterbox::Letterbox,
    opponent::{Opponent, Random},
    paint::{Component, ImageMap, Painter},
    phase::Phase,
    render::Render,
};

use js_sys::{Date, Function, Math};
use nzscq::{
    choices::{Action as NzscAction, BatchChoice, Booster, Character, DequeueChoice},
    game::BatchChoiceGame,
    outcomes::Outcome,
    scoreboard::{ActionlessPlayer, DequeueingPlayer},
};
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
    computer: Opponent<JsPrng>,
    phase: Phase,
    animation_start_secs: f64,
    has_drawn_past_completion: bool,
}

impl App {
    const IDEAL_DIMENSIONS: (u32, u32) = (1800, 1000);
    const HUMAN: usize = 0;
    const COMPUTER: usize = 1;
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
        let computer = Opponent::<JsPrng>::new(JsPrng);
        let initial_human_choices = game.choices().characters().unwrap().remove(App::HUMAN);

        let mut app = App {
            window,
            document,
            body,
            canvas,
            ctx,
            image_map,
            game,
            computer,
            phase: Phase::ChooseCharacter {
                available_characters: initial_human_choices,
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

    pub fn call_with_canvas(&self, callback: Function) -> Result<JsValue, JsValue> {
        callback.call1(&JsValue::NULL, &self.canvas)
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
        let (mut x, mut y) = (x as f64, y as f64);
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
        match action {
            click::Action::ChooseCharacter(human_character) => {
                self.handle_character_choice(human_character)
            }

            click::Action::ChooseBooster(human_booster) => {
                self.handle_booster_choice(human_booster)
            }

            click::Action::ChooseDequeue(human_dequeue) => {
                self.handle_dequeue_choice(human_dequeue)
            }

            click::Action::ChooseAction(human_action) => self.handle_action_choice(human_action),

            click::Action::StopPropagation => {}
        }
    }

    fn handle_character_choice(&mut self, human_character: Character) {
        let previously_available_characters: Vec<Character> = self
            .game
            .choices()
            .characters()
            .expect("should be able to choose character")
            .remove(App::HUMAN);
        let computer_character = self
            .computer
            .choose_character(&self.game)
            .expect("should choose character");
        let choices = BatchChoice::Characters(vec![human_character, computer_character]);

        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::CharacterPhaseDone(character_headstarts) => {
                self.phase = Phase::ChooseBooster {
                    previously_available_characters,
                    previous_outcome: character_headstarts,
                    available_boosters: self
                        .game
                        .choices()
                        .boosters()
                        .expect("should be able to choose booster")
                        .remove(App::HUMAN),
                };
            }
            Outcome::CharacterPhaseRechoose(characters) => {
                self.phase = Phase::RechooseCharacter {
                    previously_available_characters,
                    previously_mutually_chosen_character: characters[0],
                    available_characters: self
                        .game
                        .choices()
                        .characters()
                        .expect("should be able to choose character")
                        .remove(App::HUMAN),
                };
            }
            _ => panic!("outcome should be character outcome"),
        }

        self.start_animation();
    }

    fn handle_booster_choice(&mut self, human_booster: Booster) {
        let previously_available_boosters: Vec<Booster> = self
            .game
            .choices()
            .boosters()
            .expect("should be able to choose booster")
            .remove(App::HUMAN);
        let computer_booster = self
            .computer
            .choose_booster(&self.game)
            .expect("should choose booster");
        let choices = BatchChoice::Boosters(vec![human_booster, computer_booster]);

        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::BoosterPhaseDone(_) => {
                self.phase = Phase::ChooseFirstDequeue {
                    previously_available_boosters,
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .dequeueing()
                            .expect("should be dequeueing"),
                    ),
                    available_dequeues: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .dequeue_choices()
                            .expect("should be able to choose dequeue"),
                    ),
                }
            }
            _ => panic!("outcome should be booster outcome"),
        }

        self.start_animation();
    }

    fn handle_dequeue_choice(&mut self, human_dequeue: DequeueChoice) {
        let previous_scoreboard: [DequeueingPlayer; 2] = match &self.phase {
            Phase::ChooseFirstDequeue { scoreboard, .. } => scoreboard.clone(),
            Phase::ChooseSubsequentDequeue { scoreboard, .. } => scoreboard.clone(),
            _ => panic!("should be on a dequeueing phase"),
        };
        let previously_available_dequeues = self
            .game
            .choices()
            .dequeue_choices()
            .expect("should be on a dequeuing phase");

        let computer_dequeue = self
            .computer
            .choose_dequeue(&self.game)
            .expect("should choose dequeue");
        let choices = BatchChoice::DequeueChoices(vec![human_dequeue, computer_dequeue]);
        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::DequeuePhaseDone(dequeues) => {
                self.phase = Phase::ChooseAction {
                    previous_scoreboard,
                    previously_available_dequeues: helpers::vec2_to_arr2(
                        previously_available_dequeues,
                    ),
                    previous_outcome: helpers::vec2_to_arr2(dequeues),
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .actionless()
                            .expect("should be choosing actions"),
                    ),
                    available_actions: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .actions()
                            .expect("should be able to choose action"),
                    ),
                }
            }
            _ => panic!("outcome should be dequeue outcome"),
        }

        self.start_animation();
    }

    fn handle_action_choice(&mut self, human_action: NzscAction) {
        let previous_scoreboard: [ActionlessPlayer; 2] = match &self.phase {
            Phase::ChooseAction { scoreboard, .. } => scoreboard.clone(),
            _ => panic!("should be on action-choosing phase"),
        };
        let previously_available_actions = self
            .game
            .choices()
            .actions()
            .expect("should be on action-choosing phase");

        let computer_action = self
            .computer
            .choose_action(&self.game)
            .expect("should choose action");
        let choices = BatchChoice::Actions(vec![human_action, computer_action]);
        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::ActionPhaseDone(action_points_destroyed) => {
                self.phase = Phase::ChooseSubsequentDequeue {
                    previous_scoreboard,
                    previously_available_actions: helpers::vec2_to_arr2(
                        previously_available_actions,
                    ),
                    previous_outcome: helpers::vec2_to_arr2(action_points_destroyed),
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .dequeueing()
                            .expect("should be dequeueing"),
                    ),
                    available_dequeues: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .dequeue_choices()
                            .expect("should be able to choose dequeue"),
                    ),
                }
            }

            Outcome::GameOver(_) => panic!("TODO Handle GameOver"),

            _ => panic!("outcome should be action outcome"),
        }

        self.start_animation();
    }

    fn start_animation(&mut self) {
        self.animation_start_secs = helpers::millis_to_secs(Date::now());
        self.has_drawn_past_completion = false;
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
        let components = self.render();
        let ideal_dimensions = self.ideal_dimensions();
        let body_style = self.body.style();
        let mut painter = Painter::new(&self.ctx, &body_style, &self.image_map, ideal_dimensions);
        painter.paint(components)?;

        Ok(())
    }

    fn render(&self) -> Vec<Component> {
        (self.completion_factor(), &self.phase).render()
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

struct JsPrng;

impl Random for JsPrng {
    fn random(&mut self) -> f64 {
        Math::random()
    }
}
