use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, Window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<App, JsValue> {
    set_panic_hook();

    App::new()
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct App {
    window: Window,
    document: Document,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl App {
    fn new() -> Result<App, JsValue> {
        let window = web_sys::window().expect("should have a Window");
        let document = window.document().expect("should have a Document");
        let body = document.body().expect("should have a body");
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let mut app = App {
            window,
            document,
            body,
            canvas,
            ctx,
        };

        app.init()?;

        Ok(app)
    }

    fn init(&mut self) -> Result<(), JsValue> {
        self.body.append_child(&self.canvas)?;
        self.resize()?;
        Ok(())
    }

    fn resize(&mut self) -> Result<(), JsValue> {
        let (width, height) = self.dimensions()?;
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.render()?;

        Ok(())
    }

    fn dimensions(&self) -> Result<(u32, u32), JsValue> {
        let width = self.window.inner_width()?.as_f64().unwrap() as u32;
        let height = self.window.inner_height()?.as_f64().unwrap() as u32;

        Ok((width, height))
    }

    fn render(&mut self) -> Result<(), JsValue> {
        let (width, height) = self.dimensions()?;
        self.ctx.set_fill_style(&JsValue::from_str("#ff0"));
        self.ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

        Ok(())
    }
}
