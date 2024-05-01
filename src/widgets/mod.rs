use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{Context, G2d, Texture, TextureContext};

pub mod colors;
pub mod grid_view;
pub mod label;
pub mod vbox;
pub mod window;

pub trait IWidget {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>,
    );

    fn layout(&mut self, width: f64, height: f64);
}
