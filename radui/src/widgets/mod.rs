use std::cmp::max;
use std::slice::{Iter, IterMut};

use as_any::AsAny;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};

use crate::events::{Signal, SignalType};
use crate::geom::Size;

pub mod app_window;
pub mod colors;
pub mod data_grid;
pub mod file_chooser;
pub mod hbox;
pub mod label;
pub mod modal;
pub mod vbox;

pub trait IWidget: AsAny {
    fn get_x(&self) -> f64;

    fn get_y(&self) -> f64;

    fn set_x(&mut self, x: f64);

    fn set_y(&mut self, y: f64);

    fn set_width(&mut self, width: f64);

    fn set_height(&mut self, height: f64);

    fn get_width(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, w| {
            let cur = match w.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn get_height(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let size = self.get_children().fold(0u32, |acc, w| {
            let cur = match w.get_height(canvas, font) {
                Size::Absolute(h) => h,
                Size::Relative(_) => 0,
            };
            max(acc, cur)
        });
        Size::Absolute(size)
    }

    fn add_event_listener(&mut self, _typ: SignalType, _id: Vec<String>) {
        todo!()
    }

    fn get_name(&self) -> Option<&str>;

    fn get_id(&self) -> &String;

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IWidget>>;

    fn get_children(&self) -> Iter<'_, Box<dyn IWidget>>;

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for widget in self.get_children() {
            canvas.save();
            canvas.translate(widget.get_x() as f32, widget.get_y() as f32);

            widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.get_children_mut().for_each(|c| c.layout(width, height, canvas, font));
    }

    fn handle_event(&mut self, path: &mut Vec<String>, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());

        self.get_children_mut().for_each(|widget| widget.handle_event(path, event, dispatch));
        self.handle_own_event(path, event, dispatch);

        path.pop();
    }

    fn handle_own_event(
        &mut self,
        _path: &mut Vec<String>,
        _event: &Signal,
        _dispatch: &mut Box<dyn FnMut(Signal) + '_>,
    ) {
    }

    fn find_by_name(&mut self, id: &str) -> Option<&mut Box<dyn IWidget>> {
        for widget in self.get_children_mut() {
            if Some(id) == widget.get_name() {
                return Some(widget);
            }
            if let Some(child) = widget.find_by_name(id) {
                return Some(child);
            }
        }
        None
    }
}
