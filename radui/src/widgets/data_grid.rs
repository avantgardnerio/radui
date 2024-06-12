use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use uuid::Uuid;

use crate::generated::models;
use crate::geom::Size;
use crate::widgets::IWidget;

pub struct DataGrid {
    pub model: models::DataGrid,
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn IWidget>>,
}

impl IWidget for DataGrid {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, _font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn layout(&mut self, width: u32, height: u32, _canvas: &Canvas<OpenGl>, _font: &FontId) {
        self.width = width;
        self.height = height;
    }

    fn get_height(&self, _canvas: &Canvas<OpenGl>, _font: &FontId) -> Size {
        Size::Absolute(self.model
            .skinnable_container_base
            .skinnable_component
            .ui_component
            .height.unwrap() as u32)
    }

    fn get_name(&self) -> Option<&str> {
        self.model.skinnable_container_base.skinnable_component.ui_component.id.as_deref()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IWidget>> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IWidget>> {
        self.children.iter_mut()
    }

    fn get_id(&self) -> &String {
        self.model.skinnable_container_base.skinnable_component.ui_component.uid.as_ref().unwrap()
    }

    fn get_x(&self) -> f64 {
        self.model.skinnable_container_base.skinnable_component.ui_component.x.unwrap()
    }

    fn get_y(&self) -> f64 {
        self.model.skinnable_container_base.skinnable_component.ui_component.y.unwrap()
    }

    fn set_x(&mut self, x: f64) {
        self.model.skinnable_container_base.skinnable_component.ui_component.x = Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.model.skinnable_container_base.skinnable_component.ui_component.y = Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.model.skinnable_container_base.skinnable_component.ui_component.width = Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.model.skinnable_container_base.skinnable_component.ui_component.height = Some(height);
    }
}

impl From<models::DataGrid> for Box<dyn IWidget> {
    fn from(mut value: models::DataGrid) -> Self {
        value.skinnable_container_base.skinnable_component.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = DataGrid { model: value, width: 0, height: 0, children: vec![] };
        Box::new(me)
    }
}
