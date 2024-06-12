use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use uuid::Uuid;

use crate::generated::models;
use crate::generated::models::UIComponent;
use crate::widgets::IUIComponent;

pub struct DataGrid {
    pub model: models::DataGrid,
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn IUIComponent>>,
}

impl IUIComponent for DataGrid {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, _font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn layout(&mut self, width: u32, height: u32, _canvas: &Canvas<OpenGl>, _font: &FontId) {
        self.width = width;
        self.height = height;
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_model(&self) -> &UIComponent {
        &self.model.skinnable_container_base.skinnable_component.ui_component
    }

    fn get_model_mut(&mut self) -> &mut UIComponent {
        &mut self.model.skinnable_container_base.skinnable_component.ui_component
    }

    fn get_name(&self) -> Option<&str> {
        self.get_model().id.as_ref().map(|id| id.as_str())
    }

    fn get_id(&self) -> &String {
        self.get_model().uid.as_ref().unwrap()
    }
}

impl From<models::DataGrid> for Box<dyn IUIComponent> {
    fn from(mut value: models::DataGrid) -> Self {
        value.skinnable_container_base.skinnable_component.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = DataGrid { model: value, width: 0, height: 0, children: vec![] };
        Box::new(me)
    }
}
