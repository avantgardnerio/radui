use crate::generated::models;
use crate::generated::models::{WidgetChoice};
use crate::geom::{Bounds2d, Size};
use crate::widgets;
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use std::slice::IterMut;
use crate::widgets::file_chooser::FileChooser;

pub struct Window {
    pub model: models::Window,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
    pub width: u32,
    pub height: u32,
    pub file_chooser: Option<FileChooser>,
}

impl Window {
    pub fn file_chooser(&mut self) {
        if self.file_chooser.is_some() {
            return;
        }

        let mut file_chooser = FileChooser::new();

        println!("showing file dialog");
        let window = file_chooser.window.take().unwrap();
        let bounds: Bounds2d<u32> = [100, 100, 200, 200];
        let child: ([u32; 4], Box<dyn IWidget>) = (bounds, window);
        self.children.push(child);
        self.layout(self.width, self.height);
    }
}

impl IWidget for Window {
    fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        for (_bounds, c) in &mut self.children {
            c.layout(width, height);
        }
    }

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

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (bounds, c) in &self.children {
            canvas.save();
            canvas.translate(bounds[0] as f32, bounds[1] as f32);

            c.draw(canvas, &font);

            canvas.restore();
        }
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

        widgets::window::Window { model: value, children, width: 0, height: 0, file_chooser: None }
    }
}
