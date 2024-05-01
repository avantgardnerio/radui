use crate::generated::models;
use crate::widgets;
use crate::widgets::IWidget;
use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::color::WHITE;
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{clear, Context, G2d, Texture, TextureContext};

pub struct GridView {
    pub model: models::GridView,
}

impl IWidget for GridView {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>,
    ) {
        clear(WHITE, gl);
    }

    fn layout(&mut self, width: f64, height: f64) {
        todo!()
    }
}

impl From<models::GridView> for Box<dyn IWidget> {
    fn from(value: models::GridView) -> Self {
        let me = widgets::grid_view::GridView { model: value };
        Box::new(me)
    }
}
