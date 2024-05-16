use std::iter::once;
use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use itertools::{Either, Itertools};

use crate::events::{Event, Signal};
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::Size;
use crate::widgets::{IWidget, PositionedWidget};

pub struct Vbox {
    pub model: models::Vbox,
    pub children: Vec<PositionedWidget>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Vbox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, widget) in self.children.iter().enumerate() {
            let top = widget.bounds[1];
            let bottom = self.children.get(idx + 1).map(|w| w.bounds[1]).unwrap_or(self.height);
            let height = bottom - top;

            canvas.save();
            canvas.translate(0.0, top as f32);
            canvas.scissor(0.0, 0.0, self.width as f32, height as f32);

            widget.widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|w| w.widget.get_height(canvas, font)).partition_map(|h| match h {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_height: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = height as f32 - abs_height;

        // position tops
        let mut cursor = 0;
        for widget in self.children.iter_mut() {
            widget.bounds[0] = 0;
            widget.bounds[1] = cursor;
            widget.bounds[2] = width;
            let height = match widget.widget.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(h) => (h as f32 * remaining / rel_total) as u32,
            };
            cursor += height;
        }

        // layout children
        let bottoms = self.children.iter().map(|w| w.bounds[1]).skip(1).chain(once(height)).collect::<Vec<_>>();
        for (idx, widget) in self.children.iter_mut().enumerate() {
            let top = widget.bounds[1];
            let height = bottoms[idx] - top;
            widget.bounds[3] = height;
            widget.widget.layout(width, height, canvas, font);
        }
    }

    fn handle_event(&mut self, ev: &Event, signals: &mut Vec<Signal>) {
        match ev {
            Event::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let top = widget.bounds[1] as f64;
                    if pos[1] < top {
                        continue;
                    }
                    let pos = [pos[0], pos[1] - top];
                    let ev = Event::Click(pos);
                    widget.widget.handle_event(&ev, signals);
                }
            }
        }
    }

    fn get_id(&self) -> Option<&str> {
        None
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }
}

impl From<models::Vbox> for Box<dyn IWidget> {
    fn from(mut value: models::Vbox) -> Self {
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
        let me = Vbox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
