use std::slice::{Iter, IterMut};

use femtovg::{Color, Paint, Path};
use uuid::Uuid;

use crate::generated::models;
use crate::generated::models::UIComponent;
use crate::widgets::ui_component::{DrawContext, IUIComponent};

pub struct DataGrid {
    pub model: models::DataGrid,
    pub children: Vec<Box<dyn IUIComponent>>,
}

impl IUIComponent for DataGrid {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.get_width() as f32, self.get_height() as f32);
        ctx.canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn update_display_list(&mut self, width: f64, height: f64) {
        self.set_actual_size(width, height);
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

    fn measure(&mut self, _ctx: &mut DrawContext) {
        let model = self.get_model_mut();
        model.measured_width = Some(0.0);
        model.measured_height = Some(0.0);
        model.measured_min_width = Some(0.0);
        model.measured_min_height = Some(0.0);
    }
}

impl From<models::DataGrid> for Box<dyn IUIComponent> {
    fn from(mut value: models::DataGrid) -> Self {
        value.skinnable_container_base.skinnable_component.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = DataGrid { model: value, children: vec![] };
        Box::new(me)
    }
}
