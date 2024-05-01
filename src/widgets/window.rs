use femtovg::{Canvas, FontId};
use femtovg::renderer::OpenGl;
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::widgets;
use crate::widgets::IWidget;
use crate::geom::Size;

pub struct Window {
    pub model: models::Window,
    pub child: Option<Box<dyn IWidget>>,
}

impl IWidget for Window {
    fn draw(
        &self,
        canvas: &mut Canvas<OpenGl>,
        font: &FontId,
    ) {
        todo!()
    }

    fn layout(&mut self, _width: u32, _height: u32) {
        todo!()
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
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

        widgets::window::Window { model: value, child }
    }
}
