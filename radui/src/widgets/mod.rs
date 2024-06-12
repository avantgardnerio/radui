use std::slice::{Iter, IterMut};

use as_any::AsAny;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};

use crate::events::{Signal, SignalType};
use crate::generated::models;

pub mod app_window;
pub mod colors;
pub mod data_grid;
pub mod file_chooser;
pub mod hbox;
pub mod label;
pub mod modal;
pub mod vbox;

pub trait IUIComponent: AsAny {
    fn get_model(&self) -> &models::UIComponent;

    fn get_model_mut(&mut self) -> &mut models::UIComponent;

    fn get_x(&self) -> f64 {
        self.get_model().x.unwrap()
    }

    fn get_y(&self) -> f64 {
        self.get_model().y.unwrap()
    }

    fn set_x(&mut self, x: f64) {
        self.get_model_mut().x = Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.get_model_mut().y = Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.get_model_mut().width = Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.get_model_mut().height = Some(height);
    }

    fn measure(&mut self, _canvas: &Canvas<OpenGl>, _font: &FontId) {
        let model = self.get_model_mut();
        model.measured_width = Some(0.0);
        model.measured_height = Some(0.0);
        model.measured_min_width = Some(0.0);
        model.measured_min_height = Some(0.0);
    }

    fn get_width(&self) -> f64 {
        self.get_model().width.unwrap()
    }

    fn get_height(&self) -> f64 {
        self.get_model().height.unwrap()
    }

    fn add_event_listener(&mut self, _typ: SignalType, _id: Vec<String>) {
        todo!()
    }

    fn get_name(&self) -> Option<&str> {
        self.get_model().id.as_ref().map(|id| id.as_str())
    }

    fn get_id(&self) -> &String {
        self.get_model().uid.as_ref().unwrap()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>>;

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>>;

    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        for widget in self.get_children() {
            canvas.save();
            canvas.translate(widget.get_x() as f32, widget.get_y() as f32);

            widget.draw(canvas, font);

            canvas.restore();
        }
    }

    fn layout(&mut self, width: u32, height: u32, canvas: &Canvas<OpenGl>, font: &FontId) {
        self.get_children_mut().for_each(|c| {
            c.layout(width, height, canvas, font)
        });
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

    fn find_by_name(&mut self, id: &str) -> Option<&mut Box<dyn IUIComponent>> {
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
