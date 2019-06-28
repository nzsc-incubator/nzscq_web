use crate::{
    click::Action,
    colors::Rgba,
    image_map::ImageMap,
    shapes::{Circle, Rect, Translate},
};

use nzscq::choices::{ArsenalItem, Booster, Character, Move};

use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, CssStyleDeclaration, HtmlImageElement};

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
            Component::UnclickablePath {
                path,
                fill_color,
                stroke,
            } => self.paint_path(path, fill_color, stroke),
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

    fn paint_path(
        &mut self,
        path: Path,
        fill_color: Option<Rgba>,
        stroke: Option<Stroke>,
    ) -> Result<(), JsValue> {
        self.ctx.begin_path();
        self.ctx.move_to(path.start.0, path.start.1);
        for command in path.commands {
            match command {
                PathCommand::LineTo(x, y) => self.ctx.line_to(x, y),
                PathCommand::ArcTo(x1, y1, x2, y2, radius) => {
                    self.ctx.arc_to(x1, y1, x2, y2, radius)?
                }
            }
        }
        self.ctx.close_path();

        if let Some(fill_color) = fill_color {
            self.ctx
                .set_fill_style(&JsValue::from_str(&fill_color.to_upper_hash_hex()[..]));
            self.ctx.fill();
        }

        if let Some(stroke) = stroke {
            self.ctx
                .set_stroke_style(&JsValue::from_str(&stroke.color.to_upper_hash_hex()[..]));
            self.ctx.set_line_width(stroke.width);
            self.ctx.stroke();
        }

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
    UnclickablePath {
        path: Path,
        fill_color: Option<Rgba>,
        stroke: Option<Stroke>,
    },
}

impl Component {
    pub fn on_click(&self) -> Option<Action> {
        match self {
            Component::Background { .. } => None,
            Component::Rect { on_click, .. } => on_click.clone(),
            Component::Circle { on_click, .. } => on_click.clone(),
            Component::Image { on_click, .. } => on_click.clone(),
            Component::UnclickablePath { .. } => None,
        }
    }
}

impl Translate for Component {
    fn translate(&self, dx: f64, dy: f64) -> Component {
        match &self {
            Component::Background { .. } => self.clone(),
            Component::Rect {
                shape,
                fill_color,
                on_click,
            } => Component::Rect {
                shape: shape.translate(dx, dy),
                fill_color: fill_color.clone(),
                on_click: on_click.clone(),
            },
            Component::Circle {
                shape,
                fill_color,
                on_click,
            } => Component::Circle {
                shape: shape.translate(dx, dy),
                fill_color: fill_color.clone(),
                on_click: on_click.clone(),
            },
            Component::Image {
                shape,
                image_type,
                alpha,
                on_click,
            } => Component::Image {
                shape: shape.translate(dx, dy),
                image_type: *image_type,
                alpha: *alpha,
                on_click: on_click.clone(),
            },
            Component::UnclickablePath {
                path,
                fill_color,
                stroke,
            } => Component::UnclickablePath {
                path: path.translate(dx, dy),
                fill_color: fill_color.clone(),
                stroke: stroke.clone(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub start: (f64, f64),
    pub commands: Vec<PathCommand>,
}

impl Translate for Path {
    fn translate(&self, dx: f64, dy: f64) -> Path {
        Path {
            start: (self.start.0 + dx, self.start.1 + dy),
            commands: self
                .commands
                .iter()
                .map(|command| command.translate(dx, dy))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    LineTo(f64, f64),
    ArcTo(f64, f64, f64, f64, f64),
}

impl Translate for PathCommand {
    fn translate(&self, dx: f64, dy: f64) -> PathCommand {
        match self {
            &PathCommand::LineTo(x, y) => PathCommand::LineTo(x + dx, y + dy),
            &PathCommand::ArcTo(x1, y1, x2, y2, radius) => {
                PathCommand::ArcTo(x1 + dx, y1 + dy, x2 + dx, y2 + dy, radius)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stroke {
    pub color: Rgba,
    pub width: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageType {
    Character(Character),
    Booster(Booster),
    Move(Move),
    Mirror,
    Heart,
    DeclineDequeue,
    Homescreen,
    SinglePlayerButton,
    MultiPlayerButton,
}

impl From<ArsenalItem> for ImageType {
    fn from(item: ArsenalItem) -> ImageType {
        match item {
            ArsenalItem::Move(move_) => ImageType::Move(move_),
            ArsenalItem::Mirror => ImageType::Mirror,
        }
    }
}
