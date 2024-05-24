use std::iter::once;
use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use itertools::{Either, Itertools};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::{Point2d, Size};
use crate::widgets::{IWidget, PositionedWidget};

pub struct HBox {
    pub id: Uuid,
    pub model: models::Hbox,
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

    fn handle_event(&mut self, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        println!("HBox event");
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let left = widget.bounds[0];
                    if pos.dims[0] < left {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0] - left, pos.dims[1]] };
                    let ev = Signal { source: ev.source.clone(), typ: SignalType::Click(pos) };
                    widget.widget.handle_event(&ev, dispatch);
                }
            }
            _ => {}
        }
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

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl From<models::Hbox> for Box<dyn IWidget> {
    fn from(mut value: models::Hbox) -> Self {
        println!("childrec={}", value.children.len());
        let children: Vec<_> = value
            .children
            .drain(..)
            .map(|c| {
                let widget: Box<dyn IWidget> = match *c.widget_choice {
                    WidgetChoice::GridView(c) => c.into(),
                    WidgetChoice::Hbox(c) => c.into(),
                    WidgetChoice::Vbox(c) => c.into(),
                    WidgetChoice::Label(c) => c.into(),
                    WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
                };
                let bounds = [0, 0, 0, 0];
                PositionedWidget { bounds, widget }
            })
            .collect();
        let me = HBox { id: Default::default(), model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
