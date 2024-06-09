use std::collections::HashMap;
use std::iter::once;
use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::geom::Size;
use crate::widgets::{IWidget, PositionedWidget};

const FONT_SIZE: f32 = 24.0;
const PADDING: f32 = 2.0;

pub struct Label {
    pub id: Uuid,
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
    pub children: Vec<PositionedWidget>,
    pub listeners: HashMap<SignalType, Vec<Vec<Uuid>>>,
}

impl IWidget for Label {
    fn add_event_listener(&mut self, typ: SignalType, id: Vec<Uuid>) {
        self.listeners.entry(typ).and_modify(|v| v.push(id.clone())).or_insert_with(|| vec![id]);
    }

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
        let metrics = canvas.measure_text(0.0, 0.0, self.model.text_base.text.as_str(), &paint).unwrap();
        let width = metrics.height() + PADDING * 2.0;
        Size::Absolute(width as u32)
    }

    fn handle_own_event(&mut self, path: &mut Vec<Uuid>, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        let my_path = path.iter().cloned().chain(once(self.id)).collect::<Vec<_>>();
        match &event.typ {
            SignalType::Click(_pos) => {
                if let Some(listeners) = self.listeners.get(&SignalType::Activated) {
                    println!("lable lisender");
                    for listener in listeners {
                        println!("dispat lable lisender");
                        let signal =
                            Signal { source: my_path.clone(), dest: listener.clone(), typ: SignalType::Activated };
                        dispatch(signal);
                    }
                }
            }
            _ => {}
        }
    }

    fn get_name(&self) -> Option<&str> {
        self.model.name.as_deref()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl From<models::Label> for Box<dyn IWidget> {
    fn from(value: models::Label) -> Self {
        let me = Label {
            id: Uuid::new_v4(),
            model: value,
            width: 0,
            height: 0,
            children: vec![],
            listeners: Default::default(),
        };
        Box::new(me)
    }
}
