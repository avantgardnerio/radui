use crate::generated::models;
use crate::generated::models::{Components, UIComponent};
use crate::widgets::IUIComponent;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

pub struct Modal {
    pub model: models::TitleWindow,
    pub children: Vec<Box<dyn IUIComponent>>,
}

impl IUIComponent for Modal {
    fn get_model(&self) -> &UIComponent {
        &self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component
    }

    fn get_model_mut(&mut self) -> &mut UIComponent {
        &mut self.model.panel.skinnable_container.skinnable_container_base.skinnable_component.ui_component
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
        Modal { model: value, children }
    }
}
