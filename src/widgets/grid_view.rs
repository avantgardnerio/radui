use crate::generated::models;
use crate::widgets;
use crate::widgets::IWidget;
use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::color::WHITE;
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{clear, Context, G2d, rectangle, Texture, TextureContext};
use crate::geom::Size;

pub struct GridView {
    pub model: models::GridView,
    pub width: f64,
    pub height: f64,
}

impl IWidget for GridView {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        _glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>,
    ) {
        let rect = [0.0, 0.0, self.width, self.height];
        rectangle(WHITE, rect, ctx.transform, gl);
    }

    fn layout(&mut self, _width: f64, _height: f64) {
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        self.model.height.as_ref().map(|h| h.as_str()).unwrap_or("100%").parse().unwrap()
    }
}

impl From<models::GridView> for Box<dyn IWidget> {
    fn from(value: models::GridView) -> Self {
        let me = widgets::grid_view::GridView { model: value, width: 0.0, height: 0.0 };
        Box::new(me)
    }
}