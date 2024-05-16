use std::slice::{Iter, IterMut};

use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::Bounds2d;
use crate::widgets;
use crate::widgets::{IWidget, PositionedWidget};

pub trait IWindow: IWidget {
    fn get_title(&self) -> &str;

    fn get_popups_mut(&mut self) -> IterMut<'_, PositionedWidget>;

    fn get_popups(&self) -> Iter<'_, PositionedWidget>;
}

pub struct Window {
    pub model: models::Window,
    pub children: Vec<PositionedWidget>,
    pub popups: Vec<PositionedWidget>,
    pub width: u32,
    pub height: u32,
}

impl IWindow for Window {
    fn get_title(&self) -> &str {
        self.model.title.as_str()
    }

    fn get_popups_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.popups.iter_mut()
    }

    fn get_popups(&self) -> Iter<'_, PositionedWidget> {
        self.popups.iter()
    }
}

impl IWidget for Window {
    fn get_id(&self) -> Option<&str> {
        Some(self.model.id.as_ref())
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }
}

impl From<models::Window> for widgets::window::Window {
    fn from(mut value: models::Window) -> Self {
        let child = value.child.take();
        let children = child.map_or(vec![], |c| {
            let widget: Box<dyn IWidget> = match *c.widget_choice {
                WidgetChoice::GridView(c) => c.into(),
                WidgetChoice::Hbox(c) => c.into(),
                WidgetChoice::Vbox(c) => c.into(),
                WidgetChoice::Label(c) => c.into(),
                WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
            };
            let bounds: Bounds2d<u32> = [0, 0, 0, 0];
            vec![PositionedWidget { bounds, widget }]
        });

        widgets::window::Window { model: value, children, width: 0, height: 0, popups: vec![] }
    }
}
