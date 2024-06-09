use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId, Path};
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use crate::generated::models;
use crate::generated::models::Components;
use crate::widgets::{IWidget, PositionedWidget};

pub trait IWindow: IWidget {
    fn get_title(&self) -> &str;

    fn get_popups_mut(&mut self) -> IterMut<'_, PositionedWidget>;

    fn get_popups(&self) -> Iter<'_, PositionedWidget>;
}

pub struct Window {
    pub id: Uuid,
    pub model: models::WindowedApplication,
    pub children: Vec<PositionedWidget>,
    pub popups: Vec<PositionedWidget>,
    pub width: u32,
    pub height: u32,
}

impl IWindow for Window {
    fn get_title(&self) -> &str {
        self.model.title.as_ref().map(|str| str.as_str()).unwrap_or("")
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
        self.model.ui_component.id.as_ref().map(|str| str.as_str())
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

impl From<models::WindowedApplication> for Window {
    fn from(mut value: models::WindowedApplication) -> Self {
        let children = if let Some(children) = &mut value.children {
            println!("childrec={}", children.len());
            children
                .drain(..)
                .map(|child| {
                    let widget: Box<dyn IWidget> = match child {
                        Components::VBox(c) => c.into(),
                        Components::HBox(c) => c.into(),
                        Components::Label(c) => c.into(),
                        Components::DataGrid(c) => c.into(),
                        _ => unimplemented!("Not instantiable"),
                    };
                    let bounds = [0, 0, 0, 0];
                    PositionedWidget { bounds, widget }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        Window { id: Uuid::new_v4(), model: value, children, width: 0, height: 0, popups: vec![] }
    }
}
