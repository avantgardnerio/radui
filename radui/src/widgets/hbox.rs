use crate::events::Signal;
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::geom::{Bounds2d, Size};
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use itertools::{Either, Itertools};
use std::iter::once;
use std::slice::{Iter, IterMut};
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};

pub struct Hbox {
    pub model: models::Hbox,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Hbox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, (bounds, child)) in self.children.iter().enumerate() {
            let left = bounds[0];
            let right = self.children.get(idx + 1).map(|(b, _c)| b[0]).unwrap_or(self.width);
            let width = right - left;

            canvas.save();
            canvas.translate(left as f32, 0.0);
            canvas.scissor(0.0, 0.0, width as f32, self.height as f32);

            child.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|(_left, c)| c.get_width(canvas, font)).partition_map(|w| match w {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_width: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = width as f32 - abs_width;

        // position tops
        let mut cursor = 0;
        for (bounds, child) in self.children.iter_mut() {
            bounds[0] = cursor;
            bounds[1] = 0;
            bounds[3] = height;
            let width = match child.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(w) => (w as f32 * remaining / rel_total) as u32,
            };
            cursor += width;
        }

        // layout children
        let rights = self.children.iter().map(|(bounds, _c)| bounds[0]).skip(1).chain(once(width)).collect::<Vec<_>>();
        for (idx, (bounds, child)) in self.children.iter_mut().enumerate() {
            let left = bounds[0];
            let width = rights[idx] - left;
            bounds[2] = width;
            println!("Hbox bounds={bounds:?}");
            child.layout(width, width, canvas, font);
        }
    }

    fn handle_event(&mut self, ev: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>) {
        println!("HBox event");
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { .. } => {
                    for (bounds, child) in self.children.iter_mut().rev() {
                        let left = bounds[0] as f64;
                        if cursor_pos.x < left {
                            continue;
                        }
                        let pos = PhysicalPosition::new(cursor_pos.x - left, cursor_pos.y);
                        child.handle_event(ev, &pos);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn get_id(&self) -> Option<&str> {
        None
    }

    fn get_children_mut(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter()
    }
}

impl From<models::Hbox> for Box<dyn IWidget> {
    fn from(mut value: models::Hbox) -> Self {
        println!("childrec={}", value.children.len());
        let children: Vec<(Bounds2d<u32>, _)> = value
            .children
            .drain(..)
            .map(|c| {
                let child: Box<dyn IWidget> = match *c.widget_choice {
                    WidgetChoice::GridView(c) => c.into(),
                    WidgetChoice::Hbox(c) => c.into(),
                    WidgetChoice::Vbox(c) => c.into(),
                    WidgetChoice::Label(c) => c.into(),
                    WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
                };
                ([0, 0, 0, 0], child)
            })
            .collect();
        let me = Hbox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
