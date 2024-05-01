use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::widgets::IWidget;
use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{Context, G2d, Texture, TextureContext};

pub struct Vbox {
    pub model: models::Vbox,
    pub children: Vec<(f64, Box<dyn IWidget>)>,
}

impl IWidget for Vbox {
    fn draw(
        &self,
        ctx: &Context,
        gl: &mut G2d,
        glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>,
    ) {
        for (offset, child) in self.children.iter() {
            // TODO: translate by vertical offset and clip
            child.draw(ctx, gl, glyphs);
        }
    }

    fn layout(&mut self, width: f64, height: f64) {
        todo!()
    }
}

impl From<models::Vbox> for Box<dyn IWidget> {
    fn from(mut value: models::Vbox) -> Self {
        let children: Vec<(f64, _)> = value
            .children
            .drain(..)
            .map(|c| {
                let child: Box<dyn IWidget> = match *c.widget_choice {
                    WidgetChoice::GridView(c) => c.into(),
                    WidgetChoice::Vbox(c) => c.into(),
                    WidgetChoice::Label(c) => c.into(),
                    WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
                };
                (0.0, child)
            })
            .collect();
        let me = Vbox { model: value, children };
        Box::new(me)
    }
}
