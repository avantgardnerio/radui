use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::{clear, Context, G2d, Texture, TextureContext};
use piston_window::color::WHITE;
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::generated::models::GridView;
use crate::widgets::IWidget;

impl IWidget for GridView {
    fn draw(&self, ctx: &Context, gl: &mut G2d, glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>) {
        clear(WHITE, gl);
    }

    fn layout() {
        todo!()
    }
}