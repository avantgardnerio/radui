use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::{Bounds2d, Size};
use crate::widgets;
use crate::widgets::IWidget;

pub struct Window {
    pub model: models::Window,
    pub child: Option<Box<dyn IWidget>>,
}

impl IWidget for Window {
    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        todo!()
    }

    fn get_id(&self) -> Option<&str> {
        Some(self.model.id.as_ref())
    }

    fn get_children(&self) -> &[(Bounds2d<u32>, Box<dyn IWidget>)] {
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
