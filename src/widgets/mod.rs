use crate::geom::Size;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};

pub mod colors;
pub mod grid_view;
pub mod label;
pub mod vbox;
pub mod window;

pub trait IWidget {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId);

    fn layout(&mut self, width: u32, height: u32);

    fn get_width(&self) -> Size;

    fn get_height(&self) -> Size;
}
