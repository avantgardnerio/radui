use crate::geom::Size;

pub mod colors;
pub mod grid_view;
pub mod label;
pub mod vbox;
pub mod window;

pub trait IWidget {
    fn draw(
        &self,
    );

    fn layout(&mut self, width: u32, height: u32);

    fn get_width(&self) -> Size;

    fn get_height(&self) -> Size;
}
