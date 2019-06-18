use crate::{
    click::Action,
    colors::Rgba,
    shapes::{Circle, Rect},
};
use nzscq::choices::{ArsenalItem, Booster, Character, Move};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, CssStyleDeclaration, HtmlImageElement};

use std::collections::HashMap;

pub struct Painter<'a> {
    ctx: &'a CanvasRenderingContext2d,
    body_style: &'a CssStyleDeclaration,
    image_map: &'a ImageMap,
    ideal_dimensions: (u32, u32),
    backgrounds: Vec<Rgba>,
}

impl<'a> Painter<'a> {
    pub fn new(
        ctx: &'a CanvasRenderingContext2d,
        body_style: &'a CssStyleDeclaration,
        image_map: &'a ImageMap,
        ideal_dimensions: (u32, u32),
    ) -> Painter<'a> {
        Painter {
            ctx,
            body_style,
            image_map,
            ideal_dimensions,
            backgrounds: vec![],
        }
    }

    pub fn paint(&mut self, components: Vec<Component>) -> Result<(), JsValue> {
        use std::mem;

        self.backgrounds = vec![];
        for c in components {
            self.paint_component(c)?;
        }
        let backgrounds = mem::replace(&mut self.backgrounds, vec![]);
        self.update_body_background(backgrounds)
    }

    fn paint_component(&mut self, component: Component) -> Result<(), JsValue> {
        match component {
            Component::Background { color } => {
                self.paint_background(&color);
                self.backgrounds.push(color);

                Ok(())
            }
            Component::Rect {
                fill_color, shape, ..
            } => {
                self.paint_rect(fill_color, shape);

                Ok(())
            }
            Component::Circle {
                fill_color, shape, ..
            } => self.paint_circle(fill_color, shape),
            Component::Image {
                image_type,
                alpha,
                shape,
                ..
            } => self.paint_image(image_type, alpha, shape),
            Component::HealthTrapezoid {
                x,
                y,
                border_width,
                border_color,
                fill_color,
            } => self.paint_trapezoid(x, y, border_width, border_color, fill_color),
        }
    }

    fn update_body_background(&mut self, backgrounds: Vec<Rgba>) -> Result<(), JsValue> {
        let color = Rgba::composite(backgrounds);
        self.body_style
            .set_property("background-color", &color.to_upper_hash_hex()[..])
    }

    fn paint_background(&mut self, color: &Rgba) {
        let (width, height) = self.ideal_dimensions;
        self.ctx
            .set_fill_style(&JsValue::from_str(&color.to_upper_hash_hex()[..]));
        self.ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
    }

    fn paint_rect(&mut self, color: Rgba, shape: Rect) {
        let Rect {
            x,
            y,
            width,
            height,
        } = shape;
        self.ctx
            .set_fill_style(&JsValue::from_str(&color.to_upper_hash_hex()[..]));
        self.ctx.fill_rect(x, y, width, height);
    }

    fn paint_circle(&mut self, color: Rgba, shape: Circle) -> Result<(), JsValue> {
        let Circle { x, y, radius } = shape;
        self.ctx.begin_path();
        self.ctx
            .arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)?;
        self.ctx.close_path();
        self.ctx
            .set_fill_style(&JsValue::from_str(&color.to_upper_hash_hex()[..]));
        self.ctx.fill();

        Ok(())
    }

    fn paint_image(
        &mut self,
        image_type: ImageType,
        alpha: f64,
        shape: Rect,
    ) -> Result<(), JsValue> {
        let src = self.image_src(&image_type);
        let Rect {
            x,
            y,
            width,
            height,
        } = shape;

        self.ctx.set_global_alpha(alpha);
        self.ctx
            .draw_image_with_html_image_element_and_dw_and_dh(src, x, y, width, height)?;
        self.ctx.set_global_alpha(1.0);

        Ok(())
    }

    fn paint_trapezoid(
        &mut self,
        x: f64,
        y: f64,
        border_width: f64,
        border_color: Rgba,
        fill_color: Rgba,
    ) -> Result<(), JsValue> {
        self.ctx.translate(x, y)?;
        self.ctx.begin_path();
        self.ctx.move_to(80.0, 0.0);
        self.ctx.arc_to(0.0, 0.0, 30.0, 70.0, 3.0)?;
        self.ctx.arc_to(40.0, 75.0, 415.0, 70.0, 8.0)?;
        self.ctx.arc_to(400.0, 75.0, 435.0, 0.0, 8.0)?;
        self.ctx.arc_to(440.0, 0.0, 435.0, 0.0, 3.0)?;
        self.ctx.close_path();
        self.ctx.translate(-x, -y)?;

        self.ctx
            .set_stroke_style(&JsValue::from_str(&border_color.to_upper_hash_hex()[..]));
        self.ctx.set_line_width(border_width);
        self.ctx.stroke();

        self.ctx
            .set_fill_style(&JsValue::from_str(&fill_color.to_upper_hash_hex()[..]));
        self.ctx.fill();

        Ok(())
    }

    fn image_src(&self, image_type: &ImageType) -> &HtmlImageElement {
        self.image_map
            .get(&image_type)
            .expect(&format!("should have image for {:?}", image_type)[..])
    }
}

#[derive(Debug, Clone)]
pub enum Component {
    Background {
        color: Rgba,
    },
    Rect {
        fill_color: Rgba,
        shape: Rect,
        on_click: Option<Action>,
    },
    Circle {
        fill_color: Rgba,
        shape: Circle,
        on_click: Option<Action>,
    },
    Image {
        image_type: ImageType,
        alpha: f64,
        shape: Rect,
        on_click: Option<Action>,
    },
    HealthTrapezoid {
        x: f64,
        y: f64,
        border_width: f64,
        border_color: Rgba,
        fill_color: Rgba,
    },
}

impl Component {
    pub fn on_click(&self) -> Option<Action> {
        match self {
            Component::Background { .. } => None,
            Component::Rect { on_click, .. } => on_click.clone(),
            Component::Circle { on_click, .. } => on_click.clone(),
            Component::Image { on_click, .. } => on_click.clone(),
            Component::HealthTrapezoid { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageType {
    Character(Character),
    Booster(Booster),
    Move(Move),
    Mirror,
    Heart,
    DeclineDequeue,
}

impl ImageType {
    pub fn from_arsenal_item(item: ArsenalItem) -> ImageType {
        match item {
            ArsenalItem::Move(move_) => ImageType::Move(move_),
            ArsenalItem::Mirror => ImageType::Mirror,
        }
    }
}

pub type ImageMap = HashMap<ImageType, HtmlImageElement>;
