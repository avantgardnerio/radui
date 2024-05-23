use std::cmp::max;
use std::slice::{Iter, IterMut};

use as_any::AsAny;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};

use crate::events::{Signal, SignalType};
use crate::geom::{Bounds2d, Size};

pub mod colors;
pub mod file_chooser;
pub mod grid_view;
mod hbox;
pub mod label;
pub mod vbox;
pub mod window;

pub struct PositionedWidget {
    pub bounds: Bounds2d<u32>,
    pub widget: Box<dyn IWidget>,
}

pub trait IWidget: AsAny {
    fn get_width(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, w| {
            let cur = match w.widget.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn get_height(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, w| {
            let cur = match w.widget.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn add_event_listener(&mut self, _typ: SignalType) {
        todo!()
    }

    fn get_id(&self) -> Option<&str>;

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget>;

    fn get_children(&self) -> Iter<'_, PositionedWidget>;

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for widget in self.get_children() {
            canvas.save();
            canvas.translate(widget.bounds[0] as f32, widget.bounds[1] as f32);

            widget.widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        for widget in &mut self.get_children_mut() {
            widget.widget.layout(width, height, canvas, font);
        }
    }

    fn handle_event(&mut self, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        self.get_children_mut().for_each(|widget| widget.widget.handle_event(event, dispatch));
    }

    fn find_by_id(&mut self, id: &str) -> Option<&mut Box<dyn IWidget>> {
        for widget in self.get_children_mut() {
            if Some(id) == widget.widget.get_id() {
                return Some(&mut widget.widget);
            }
            if let Some(child) = widget.widget.find_by_id(id) {
                return Some(child);
            }
        }
        None
    }
}
