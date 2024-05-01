use gfx_device_gl::{CommandBuffer, Factory, Resources};
use piston_window::{Context, G2d, Texture, TextureContext};
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::widgets;
use crate::widgets::IWidget;

pub struct Window {
    pub model: models::Window,
    pub child: Option<Box<dyn IWidget>>,
}

impl IWidget for Window {
    fn draw(&self, ctx: &Context, gl: &mut G2d, glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>) {
        todo!()
    }

    fn layout(&mut self, width: f64, height: f64) {
        todo!()
    }
}

impl From<models::Window> for widgets::window::Window {
    fn from(mut value: models::Window) -> Self {
        let child = value.child.take();
        let child = child.map(|c| {
            let child: Box<dyn IWidget> = match *c.widget_choice {
                WidgetChoice::GridView(c) => c.into(),
                WidgetChoice::Vbox(c) => c.into(),
                WidgetChoice::Label(c) => c.into(),
                WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
            };
            child
        });
        let me = widgets::window::Window {
            model: value,
            child,
        };
        me
    }
}
