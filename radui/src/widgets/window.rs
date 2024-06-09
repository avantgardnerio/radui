use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use crate::generated::models;
use crate::geom::Bounds2d;
use crate::widgets::{IWidget, PositionedWidget};

pub trait IWindow: IWidget {
    fn get_title(&self) -> &str;

    fn get_popups_mut(&mut self) -> IterMut<'_, PositionedWidget>;

    fn get_popups(&self) -> Iter<'_, PositionedWidget>;
}

pub struct Window {
    pub id: Uuid,
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
    fn get_name(&self) -> Option<&str> {
        Some(self.model.name.as_ref())
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        println!("window width = {width}");
        self.width = width;
        self.height = height;
        self.get_children_mut().for_each(|c| c.widget.layout(width, height, canvas, font));
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for widget in self.get_children() {
            canvas.save();
            canvas.translate(widget.bounds[0] as f32, widget.bounds[1] as f32);

            widget.widget.draw(canvas, font);

            canvas.restore();
        }

        println!("drawing window");
        let mut first = true;
        for popup in self.get_popups() {
            println!("drawing popups");
            if first {
                first = false;
                let mut path = Path::new();
                path.rect(0.0, 0.0, self.width as f32, self.height as f32);
                // canvas.fill_path(&path, &Paint::color(Color::rgba(0, 0, 0, 128)));
            }
        }
    }
}

impl From<models::Window> for Window {
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

        Window { id: Uuid::new_v4(), model: value, children, width: 0, height: 0, popups: vec![] }
    }
}
