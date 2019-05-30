use crate::shapes::{Circle, Rect};
use nzscq::choices::{Booster, Character, Move};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use std::collections::HashMap;

pub struct Painter<'a> {
    ctx: &'a mut CanvasRenderingContext2d,
    image_map: &'a ImageMap,
    ideal_dimensions: (u32, u32),
    dimensions: (u32, u32),
}

impl<'a> Painter<'a> {
    pub fn new(
        ctx: &'a mut CanvasRenderingContext2d,
        image_map: &'a ImageMap,
        ideal_dimensions: (u32, u32),
        dimensions: (u32, u32),
    ) -> Painter<'a> {
        Painter {
            ctx,
            image_map,
            ideal_dimensions,
            dimensions,
        }
    }

    pub fn paint(&mut self, components: Vec<Component>) -> Result<(), JsValue> {
        for c in components {
            self.paint_component(c)?;
        }

        Ok(())
    }

    fn paint_component(&mut self, component: Component) -> Result<(), JsValue> {
        match component {
            Component::Background(color) => self.paint_background(color),
            Component::Rect { fill_color, shape } => self.paint_rect(fill_color, shape),
            Component::Circle { fill_color, shape } => self.paint_circle(fill_color, shape),
            Component::Image { image_type, shape } => self.paint_image(image_type, shape),
        }
    }

    fn paint_background(&mut self, color: &str) -> Result<(), JsValue> {
        self.disable_letterbox()?;
        self.ctx.set_fill_style(&JsValue::from_str(color));
        let (width, height) = self.dimensions;
        self.ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

        Ok(())
    }

    fn paint_rect(&mut self, color: &str, shape: Rect) -> Result<(), JsValue> {
        let Rect {
            x,
            y,
            width,
            height,
        } = shape;
        self.enable_letterbox()?;
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill_rect(x, y, width, height);

        Ok(())
    }

    fn paint_circle(&mut self, color: &str, shape: Circle) -> Result<(), JsValue> {
        let Circle { x, y, radius } = shape;
        self.enable_letterbox()?;
        self.ctx.begin_path();
        self.ctx.arc(x, y, radius, 0.0, std::f64::consts::PI)?;
        self.ctx.close_path();
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill();

        Ok(())
    }

    fn paint_image(&mut self, image_type: ImageType, shape: Rect) -> Result<(), JsValue> {
        self.enable_letterbox()?;
        let src = self.image_src(&image_type);
        let Rect {
            x,
            y,
            width,
            height,
        } = shape;
        self.ctx
            .draw_image_with_html_image_element_and_dw_and_dh(src, x, y, width, height)
    }

    fn image_src(&self, image_type: &ImageType) -> &HtmlImageElement {
        self.image_map.get(&image_type).unwrap()
    }

    fn enable_letterbox(&mut self) -> Result<(), JsValue> {
        self.ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
        if self.aspect() > self.ideal_aspect() {
            let (actual_width, actual_height) = self.dimensions;
            let (actual_width, actual_height) = (actual_width as f64, actual_height as f64);
            let (ideal_width, ideal_height) = (
                self.ideal_dimensions.0 as f64,
                self.ideal_dimensions.1 as f64,
            );
            let scale = actual_height / ideal_height;
            self.ctx
                .translate((actual_width - scale * ideal_width) / 2.0, 0.0)?;
            self.ctx.scale(scale, scale)
        } else {
            let (actual_width, actual_height) = self.dimensions;
            let (actual_width, actual_height) = (actual_width as f64, actual_height as f64);
            let (ideal_width, ideal_height) = (
                self.ideal_dimensions.0 as f64,
                self.ideal_dimensions.1 as f64,
            );
            let scale = actual_width / ideal_width;
            self.ctx
                .translate(0.0, (actual_height - scale * ideal_height) / 2.0)?;
            self.ctx.scale(scale, scale)
        }
    }

    fn aspect(&self) -> f64 {
        let (width, height) = self.dimensions;
        width as f64 / height as f64
    }

    fn ideal_aspect(&self) -> f64 {
        let (width, height) = self.ideal_dimensions;
        width as f64 / height as f64
    }

    fn disable_letterbox(&mut self) -> Result<(), JsValue> {
        self.ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub enum Component {
    Background(&'static str),
    Rect {
        fill_color: &'static str,
        shape: Rect,
    },
    Circle {
        fill_color: &'static str,
        shape: Circle,
    },
    Image {
        image_type: ImageType,
        shape: Rect,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageType {
    Character(Character),
    Booster(Booster),
    Move(Move),
    Mirror,
}

pub type ImageMap = HashMap<ImageType, HtmlImageElement>;
