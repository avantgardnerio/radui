use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::geom::{Bounds2d, Size};
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use std::slice::{Iter, IterMut};
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};

const FONT_SIZE: f32 = 22.0;
const PADDING: f32 = 2.0;

pub struct Label {
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
}

impl IWidget for Label {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::rgb(246, 245, 244)));

        let mut paint = Paint::color(Color::black());
        paint.set_font(&[*font]);
        paint.set_font_size(FONT_SIZE);
        canvas.fill_text(0.0, FONT_SIZE, self.model.text.as_str(), &paint).expect("Can't write");
    }

    fn layout(&mut self, width: u32, height: u32, _canvas: &Canvas<OpenGl>, _font: &FontId) {
        self.width = width;
        self.height = height;
    }

    fn get_width(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let mut paint = Paint::color(Color::black());
        paint.set_font(&[*font]);
        paint.set_font_size(FONT_SIZE);
        let metrics = canvas.measure_text(0.0, 0.0, self.model.text.as_str(), &paint).unwrap();
        let width = metrics.width() + PADDING * 2.0;
        Size::Absolute(width as u32)
    }

    fn get_height(&self, canvas: &Canvas<OpenGl>, font: &FontId) -> Size {
        let mut paint = Paint::color(Color::black());
        paint.set_font(&[*font]);
        paint.set_font_size(FONT_SIZE);
        let metrics = canvas.measure_text(0.0, 0.0, self.model.text.as_str(), &paint).unwrap();
        let width = metrics.height() + PADDING * 2.0;
        Size::Absolute(width as u32)
    }

    fn handle_event(&mut self, event: &Event<'_, ()>, _cursor_pos: &PhysicalPosition<f64>) -> Option<Signal> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { .. } => {
                    let signal = Signal { source: self.get_id().unwrap_or("").to_string(), typ: SignalType::Activated };
                    return Some(signal);
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn get_id(&self) -> Option<&str> {
        self.model.id.as_ref().map(|s| s.as_str())
    }

    fn get_children(&self) -> Iter<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }
}

impl From<models::Label> for Box<dyn IWidget> {
    fn from(value: models::Label) -> Self {
        let me = Label { model: value, width: 0, height: 0, children: vec![] };
        Box::new(me)
    }
}
