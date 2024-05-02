use crate::generated::models;
use crate::geom::{Bounds2d, Size};
use crate::widgets;
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use std::slice::IterMut;
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};

pub struct GridView {
    pub model: models::GridView,
    pub width: u32,
    pub height: u32,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
}

impl IWidget for GridView {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, _font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        self.model.height.as_ref().map(|h| h.as_str()).unwrap_or("100%").parse().unwrap()
    }

    fn handle_event(&mut self, event: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    println!("grid {state:?} {button:?} {cursor_pos:?}");
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn get_id(&self) -> Option<&str> {
        self.model.id.as_ref().map(|s| s.as_str())
    }

    fn get_children(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }
}

impl From<models::GridView> for Box<dyn IWidget> {
    fn from(value: models::GridView) -> Self {
        let me = GridView { model: value, width: 0, height: 0, children: vec![] };
        Box::new(me)
    }
}
