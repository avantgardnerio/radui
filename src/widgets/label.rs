use piston_window::{clear, Context, G2d, Text, Texture, TextureContext, Transformed};
use piston_window::glyph_cache::rusttype::GlyphCache;
use gfx_device_gl::{Factory, Resources, CommandBuffer};
use piston_window::color::BLACK;
use crate::generated::models;
use crate::widgets::IWidget;

const MENU_BACKGROUND: [f32; 4] = [246.0 / 255.0, 245.0 / 255.0, 244.0 / 255.0, 1.0];
const FONT_SIZE: u32 = 14;

pub struct Label {
    pub model: models::Label,
}

impl IWidget for Label {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>
    ) {
        println!("drawing label");
        clear(MENU_BACKGROUND, gl);
        let transform = ctx.transform.trans(0.0, FONT_SIZE as f64);
        Text::new_color(BLACK, FONT_SIZE)
            .draw(&self.model.text, glyphs, &ctx.draw_state, transform, gl)
            .unwrap();
    }

    fn layout(&mut self, width: f64, height: f64) {
        todo!()
    }
}

impl From<models::Label> for Box<dyn IWidget> {
    fn from(value: models::Label) -> Self {
        let me = Label { model: value };
        Box::new(me)
    }
}
