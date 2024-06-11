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
use crate::widgets::{IWidget, PositionedWidget};

pub struct HBox {
    pub model: models::HBox,
    pub children: Vec<PositionedWidget>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for HBox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, widget) in self.children.iter().enumerate() {
            let left = widget.bounds[0];
            let right = self.children.get(idx + 1).map(|w| w.bounds[0]).unwrap_or(self.width);
            let width = right - left;

            canvas.save();
            canvas.translate(left as f32, 0.0);
            canvas.scissor(0.0, 0.0, width as f32, self.height as f32);

            widget.widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|w| w.widget.get_width(canvas, font)).partition_map(|w| match w {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_width: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = width as f32 - abs_width;

        // position tops
        let mut cursor = 0;
        for widget in self.children.iter_mut() {
            widget.bounds[0] = cursor;
            widget.bounds[1] = 0;
            widget.bounds[3] = height;
            let width = match widget.widget.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(w) => (w as f32 * remaining / rel_total) as u32,
            };
            cursor += width;
        }

        // layout children
        let rights = self.children.iter().map(|w| w.bounds[0]).skip(1).chain(once(width)).collect::<Vec<_>>();
        for (idx, widget) in self.children.iter_mut().enumerate() {
            let left = widget.bounds[0];
            let width = rights[idx] - left;
            widget.bounds[2] = width;
            println!("Hbox bounds={:?}", widget.bounds);
            widget.widget.layout(width, width, canvas, font);
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let left = widget.bounds[0];
                    if pos.dims[0] < left {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0] - left, pos.dims[1]] };
                    let ev = Signal { typ: SignalType::Click(pos), ..ev.clone() };
                    widget.widget.handle_event(path, &ev, dispatch);
                }
            }
            _ => {}
        }
        path.pop();
    }

    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }

    fn get_id(&self) -> &String {
        self.model.mx_box.container.ui_component.uid.as_ref().unwrap()
    }
}

impl From<models::HBox> for Box<dyn IWidget> {
    fn from(mut value: models::HBox) -> Self {
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
                let bounds = [0, 0, 0, 0];
                PositionedWidget { bounds, widget }
            })
            .collect::<Vec<_>>();
        value.mx_box.container.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = HBox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
