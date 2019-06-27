mod app;
mod canvas_dimensions;
mod click;
mod colors;
mod helpers;
mod letterbox;
mod opponent;
mod paint;
mod phase;
mod render;
mod shapes;
mod side;
mod state;

use app::App;

use js_sys::Function;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(get_move_images: Function) -> Result<App, JsValue> {
    set_panic_hook();

    App::new(get_move_images)
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
