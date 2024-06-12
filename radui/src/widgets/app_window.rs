use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId, Path};
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use crate::generated::models;
use crate::generated::models::Components;
use crate::widgets::IUIComponent;

pub trait IAppWindow: IUIComponent {
    fn get_title(&self) -> &str;

    fn get_popups_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>>;

    fn get_popups(&self) -> Iter<'_, Box<dyn IUIComponent>>;
}

pub struct AppWindow {
    pub model: models::WindowedApplication,
    pub children: Vec<Box<dyn IUIComponent>>,
    pub popups: Vec<Box<dyn IUIComponent>>,
    pub width: u32,
    pub height: u32,
}

impl IAppWindow for AppWindow {
    fn get_title(&self) -> &str {
        self.model.title.as_ref().map(|str| str.as_str()).unwrap_or("")
    }

    fn get_popups_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.popups.iter_mut()
    }

    fn get_popups(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.popups.iter()
    }
}

impl IUIComponent for AppWindow {
    fn get_name(&self) -> Option<&str> {
        self.model
            .application
            .skinnable_container
            .skinnable_container_base
            .skinnable_component
            .ui_component
            .id
            .as_ref()
            .map(|str| str.as_str())
    }

    fn get_id(&self) -> &String {
        self.model
            .application
            .skinnable_container
            .skinnable_container_base
            .skinnable_component
            .ui_component
            .uid
            .as_ref()
            .unwrap()
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        println!("window width = {width}");
        self.width = width;
        self.height = height;
        self.get_children_mut().for_each(|c| {
            c.layout(width, height, canvas, font)
        });
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for widget in self.get_children() {
            canvas.save();
            canvas.translate(widget.get_x() as f32, widget.get_y() as f32);

            widget.draw(canvas, font);

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

    fn get_x(&self) -> f64 {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.x.unwrap()
    }

    fn get_y(&self) -> f64 {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.y.unwrap()
    }

    fn set_x(&mut self, x: f64) {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.x =
            Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.y =
            Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.width =
            Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.height =
            Some(height);
    }
}

impl From<models::WindowedApplication> for AppWindow {
    fn from(mut value: models::WindowedApplication) -> Self {
        println!("childrec={}", value.children.len());
        let children = value
            .children
            .drain(..)
            .map(|child| {
                let widget: Box<dyn IUIComponent> = match child {
                    Components::VBox(c) => c.into(),
                    Components::HBox(c) => c.into(),
                    Components::Label(c) => c.into(),
                    Components::DataGrid(c) => c.into(),
                    _ => unimplemented!("Not instantiable"),
                };
                widget
            })
            .collect::<Vec<_>>();
        value.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.uid =
            Some(Uuid::new_v4().to_string());
        AppWindow { model: value, children, width: 0, height: 0, popups: vec![] }
    }
}
