use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::{Bounds2d, Size};
use crate::widgets;
use crate::widgets::IWidget;
use std::slice::IterMut;

pub struct Window {
    pub model: models::Window,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
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

    fn get_children(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }
}

impl From<models::Window> for widgets::window::Window {
    fn from(mut value: models::Window) -> Self {
        let child = value.child.take();
        let children = child.map_or(vec![], |c| {
            let child: Box<dyn IWidget> = match *c.widget_choice {
                WidgetChoice::GridView(c) => c.into(),
                WidgetChoice::Vbox(c) => c.into(),
                WidgetChoice::Label(c) => c.into(),
                WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
            };
            let bounds: Bounds2d<u32> = [0, 0, 0, 0];
            vec![(bounds, child)]
        });

        widgets::window::Window { model: value, children }
    }
}
