use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::{Context, G2d, Texture, TextureContext};
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::generated::models::Widget;

mod label;
mod colors;
mod vbox;
mod grid_view;

pub trait IWidget {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>
    );

    fn layout();
}

impl IWidget for Widget {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>
    ) {
        println!("drawing widget");
    }

    fn layout() {
    }
}
