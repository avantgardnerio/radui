use crate::events::Signal;
use crate::geom::{Bounds2d, Size};
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use std::slice::IterMut;
use winit::dpi::PhysicalPosition;
use winit::event::Event;

pub mod colors;
pub mod grid_view;
pub mod label;
pub mod vbox;
pub mod window;

pub trait IWidget {
    fn get_width(&self) -> Size;

    fn get_height(&self) -> Size;

    fn get_id(&self) -> Option<&str>;

    fn get_children(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)>;

    fn draw(&self, _canvas: &mut Canvas<OpenGl>, _font: &FontId) {}

    fn layout(&mut self, _width: u32, _height: u32) {}

    fn handle_event(&mut self, _event: &Event<'_, ()>, _cursor_pos: &PhysicalPosition<f64>) -> Option<Signal> {
        None
    }

    fn find_by_id(&mut self, id: &str) -> Option<&mut Box<dyn IWidget>> {
        for (_bounds, child) in self.get_children() {
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
