use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::{Context, G2d, Texture, TextureContext};
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::generated::models::{Vbox, WidgetChoice};
use crate::widgets::IWidget;

impl IWidget for Vbox {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>
    ) {
        for child in self.children.iter() {
            match child.widget_choice.as_ref() {
                WidgetChoice::GridView(c) => {},
                WidgetChoice::Vbox(c) => c.draw(ctx, gl, glyphs),
                WidgetChoice::Label(c) => c.draw(ctx, gl, glyphs),
                WidgetChoice::GridView(c) => c.draw(ctx, gl, glyphs),
                WidgetChoice::__Unknown__(_) => {}
            }
        }
    }

    fn layout() {
        todo!()
    }
}