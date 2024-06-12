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
use crate::widgets::IWidget;

pub struct Vbox {
    pub model: models::VBox,
    pub children: Vec<Box<dyn IWidget>>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Vbox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, widget) in self.children.iter().enumerate() {
            let top = widget.get_y();
            let bottom = self.children.get(idx + 1).map(|w| w.get_y()).unwrap_or(self.height as f64);
            let height = bottom - top;

            canvas.save();
            canvas.translate(0.0, top as f32);
            canvas.scissor(0.0, 0.0, self.width as f32, height as f32);

            widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|w| w.get_height(canvas, font)).partition_map(|h| match h {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_height: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = height as f32 - abs_height;

        // position tops
        let mut cursor = 0;
        for widget in self.children.iter_mut() {
            widget.set_x(0.0);
            widget.set_y(cursor as f64);
            widget.set_width(width as f64);
            let height = match widget.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(h) => (h as f32 * remaining / rel_total) as u32,
            };
            cursor += height;
        }

        // layout children
        let bottoms = self.children.iter().map(|w| w.get_y()).skip(1).chain(once(height as f64)).collect::<Vec<_>>();
        for (idx, widget) in self.children.iter_mut().enumerate() {
            let top = widget.get_y();
            let height = bottoms[idx] - top;
            widget.set_height(height);
            widget.layout(width, height as u32, canvas, font);
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let top = widget.get_y();
                    if pos.dims[1] < top as u32 {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0], pos.dims[1] - top as u32] };
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

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IWidget>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IWidget>> {
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
        self.model.mx_box.container.ui_component.x = Some(x)
    }

    fn set_y(&mut self, y: f64) {
        self.model.mx_box.container.ui_component.y = Some(y)
    }

    fn set_width(&mut self, width: f64) {
        self.model.mx_box.container.ui_component.width = Some(width)
    }

    fn set_height(&mut self, height: f64) {
        self.model.mx_box.container.ui_component.height = Some(height)
    }
}

impl From<models::VBox> for Box<dyn IWidget> {
    fn from(mut value: models::VBox) -> Self {
        println!("childrec={}", value.children.len());
        let children = value
            .children
            .drain(..)
            .map(|child| {
                let widget: Box<dyn IWidget> = match child {
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
        let me = Vbox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
