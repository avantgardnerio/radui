use crate::generated::models;
use crate::generated::models::Components;
use crate::widgets::{IWidget, PositionedWidget};
use std::slice::{Iter, IterMut};
use uuid::Uuid;

pub struct Modal {
    pub model: models::TitleWindow,
    pub children: Vec<PositionedWidget>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Modal {
    fn get_name(&self) -> Option<&str> {
        todo!()
    }

    fn get_id(&self) -> &String {
        self.model.ui_component.uid.as_ref().unwrap()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        todo!()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        todo!()
    }
}

impl From<models::TitleWindow> for Modal {
    fn from(mut value: models::TitleWindow) -> Self {
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
        value.ui_component.uid = Some(Uuid::new_v4().to_string());
        Modal { model: value, children, width: 0, height: 0 }
    }
}
