use std::iter::once;
use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use itertools::{Either, Itertools};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::Components;
use crate::geom::{Point2d, Size};
use crate::widgets::IUIComponent;

pub struct HBox {
    pub model: models::HBox,
    pub children: Vec<Box<dyn IUIComponent>>,
    pub width: f64,
    pub height: f64,
}

impl IUIComponent for HBox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, widget) in self.children.iter().enumerate() {
            let left = widget.get_x();
            let right = self.children.get(idx + 1).map(|w| w.get_x()).unwrap_or(self.width);
            let width = right - left;

            canvas.save();
            canvas.translate(left as f32, 0.0);
            canvas.scissor(0.0, 0.0, width as f32, self.height as f32);

            widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width as f64;
        self.height = height as f64;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|w| w.get_width(canvas, font)).partition_map(|w| match w {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_width: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = width as f32 - abs_width;

        // position tops
        let mut cursor = 0;
        for widget in self.children.iter_mut() {
            widget.set_x(cursor as f64);
            widget.set_y(0 as f64);
            widget.set_width(height as f64);
            let width = match widget.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(w) => (w as f32 * remaining / rel_total) as u32,
            };
            cursor += width;
        }

        // layout children
        let rights = self.children.iter().map(|w| w.get_x()).skip(1).chain(once(width as f64)).collect::<Vec<_>>();
        for (idx, widget) in self.children.iter_mut().enumerate() {
            let left = widget.get_x();
            let width = rights[idx] - left;
            widget.set_width(width);
            // println!("Hbox bounds={:?}", widget.bounds);
            widget.layout(width as u32, width as u32, canvas, font);
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let left = widget.get_x();
                    if pos.dims[0] < left as u32 {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0] - left as u32, pos.dims[1]] };
                    let ev = Signal { typ: SignalType::Click(pos), ..ev.clone() };
                    widget.handle_event(path, &ev, dispatch);
                }
            }
            _ => {}
        }
        path.pop();
    }

    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn get_id(&self) -> &String {
        self.model.mx_box.container.ui_component.uid.as_ref().unwrap()
    }

    fn get_x(&self) -> f64 {
        self.model.mx_box.container.ui_component.x.unwrap()
    }

    fn get_y(&self) -> f64 {
        self.model.mx_box.container.ui_component.y.unwrap()
    }

    fn set_x(&mut self, x: f64) {
        self.model.mx_box.container.ui_component.x = Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.model.mx_box.container.ui_component.y = Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.model.mx_box.container.ui_component.width = Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.model.mx_box.container.ui_component.height = Some(height);
    }
}

impl From<models::HBox> for Box<dyn IUIComponent> {
    fn from(mut value: models::HBox) -> Self {
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
        value.mx_box.container.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = HBox { model: value, children, width: 0.0, height: 0.0 };
        Box::new(me)
    }
}
