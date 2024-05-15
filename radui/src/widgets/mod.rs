use crate::events::Signal;
use crate::geom::{Bounds2d, Size};
use as_any::AsAny;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use std::cmp::max;
use std::slice::{Iter, IterMut};
use winit::dpi::PhysicalPosition;
use winit::event::Event;

pub mod colors;
pub mod file_chooser;
pub mod grid_view;
mod hbox;
pub mod label;
pub mod vbox;
pub mod window;

pub trait IWidget: AsAny {
    fn get_width(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, cur| {
            let (_bounds, widget) = cur;
            let cur = match widget.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn get_height(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, cur| {
            let (_bounds, widget) = cur;
            let cur = match widget.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn get_id(&self) -> Option<&str>;

    fn get_children_mut(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)>;

    fn get_children(&self) -> Iter<'_, (Bounds2d<u32>, Box<dyn IWidget>)>;

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for (bounds, c) in self.get_children() {
            canvas.save();
            canvas.translate(bounds[0] as f32, bounds[1] as f32);

            c.draw(canvas, &font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        for (_bounds, c) in &mut self.get_children_mut() {
            c.layout(width, height, canvas, font);
        }
    }

    fn handle_event(&mut self, event: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>) -> Option<Signal> {
        for (_bounds, child) in self.get_children_mut() {
            if let Some(signal) = child.handle_event(event, cursor_pos) {
                return Some(signal);
            }
        }
        None
    }

    fn find_by_id(&mut self, id: &str) -> Option<&mut Box<dyn IWidget>> {
        for (_bounds, child) in self.get_children_mut() {
            if Some(id) == child.get_id() {
                return Some(child);
            }
            if let Some(child) = child.find_by_id(id) {
                return Some(child);
            }
        }
        None
    }
}
