use crate::generated::models;
use crate::generated::models::Components;
use crate::widgets::IUIComponent;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

pub struct Modal {
    pub model: models::TitleWindow,
    pub children: Vec<Box<dyn IUIComponent>>,
    pub width: u32,
    pub height: u32,
}

impl IUIComponent for Modal {
    fn get_x(&self) -> f64 {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.x.unwrap()
    }

    fn get_y(&self) -> f64 {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.y.unwrap()
    }

    fn set_x(&mut self, x: f64) {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.x = Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.y = Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.width =
            Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.height =
            Some(height);
    }

    fn get_name(&self) -> Option<&str> {
        self.model
            .panel
            .skinnable_container
            .skinnable_container_base
            .skinnable_component
            .ui_component
            .id
            .as_ref()
            .map(|id| id.as_str())
    }

    fn get_id(&self) -> &String {
        self.model
            .panel
            .skinnable_container
            .skinnable_container_base
            .skinnable_component
            .ui_component
            .uid
            .as_ref()
            .unwrap()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        todo!()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        todo!()
    }
}

impl From<models::TitleWindow> for Modal {
    fn from(mut value: models::TitleWindow) -> Self {
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
        value.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component.uid =
            Some(Uuid::new_v4().to_string());
        Modal { model: value, children, width: 0, height: 0 }
    }
}
