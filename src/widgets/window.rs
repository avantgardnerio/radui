use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::Size;
use crate::widgets;
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use winit::dpi::PhysicalPosition;
use winit::event::Event;

pub struct Window {
    pub model: models::Window,
    pub child: Option<Box<dyn IWidget>>,
}

impl IWidget for Window {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
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

    fn handle_event(&mut self, event: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>) {}
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
