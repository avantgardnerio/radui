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

pub struct Vbox {
    pub model: models::Vbox,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Vbox {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (idx, (bounds, child)) in self.children.iter().enumerate() {
            let top = bounds[1];
            let bottom = self.children.get(idx + 1).map(|(b, _c)| b[1]).unwrap_or(self.height);
            let height = bottom - top;

            canvas.save();
            canvas.translate(0.0, top as f32);
            canvas.scissor(0.0, 0.0, self.width as f32, height as f32);

            child.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|(_top, c)| c.get_height(canvas, font)).partition_map(|h| match h {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_height: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = height as f32 - abs_height as f32;

        // position tops
        let mut cursor = 0;
        for (bounds, child) in self.children.iter_mut() {
            bounds[0] = 0;
            bounds[1] = cursor;
            bounds[2] = width;
            let height = match child.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(h) => (h as f32 * remaining / rel_total) as u32,
            };
            cursor += height;
        }

        // layout children
        let bottoms =
            self.children.iter().map(|(bounds, _c)| bounds[1]).skip(1).chain(once(height)).collect::<Vec<_>>();
        for (idx, (bounds, child)) in self.children.iter_mut().enumerate() {
            let top = bounds[1];
            let height = bottoms[idx] - top;
            bounds[3] = height;
            println!("bounds={bounds:?}");
            child.layout(width, height, canvas, font);
        }
    }

    fn handle_event(&mut self, ev: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>) -> Option<Signal> {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { .. } => {
                    for (bounds, child) in self.children.iter_mut().rev() {
                        let top = bounds[1] as f64;
                        if cursor_pos.y < top {
                            continue;
                        }
                        let pos = PhysicalPosition::new(cursor_pos.x, cursor_pos.y - top);
                        return child.handle_event(ev, &pos);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
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

impl From<models::Vbox> for Box<dyn IWidget> {
    fn from(mut value: models::Vbox) -> Self {
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
        let me = Vbox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
