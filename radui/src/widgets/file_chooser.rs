use crate::events::{Signal, SignalType};
use crate::generated::models::Windows;
use crate::geom::Bounds2d;
use crate::widgets;
use crate::widgets::label::Label;
use crate::widgets::window::Window;
use crate::widgets::{window, IWidget};
use std::env;
use std::path::PathBuf;
use std::slice::{Iter, IterMut};
use winit::dpi::PhysicalPosition;
use winit::event::Event;
use yaserde::de::from_str;

pub struct FileChooser {
    pub id: String,
    pub current_dir: PathBuf,
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
}

impl FileChooser {
    pub fn new(id: &str) -> Self {
        println!("new FC");
        let bytes = include_bytes!("../../resources/lib.xml");
        let content = String::from_utf8_lossy(bytes);
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.id == "file_chooser").unwrap();
        let window = windows.window.remove(idx);
        let mut window: widgets::window::Window = window.into();

        let current_dir = env::current_dir().unwrap();
        let lbl_path = window.find_by_id("lblPath").unwrap();
        let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
        lbl_path.model.text = current_dir.to_str().unwrap().to_string();

        let label = window.find_by_id("lblUp").unwrap();
        label.add_event_listener(SignalType::Activated);

        Self { id: id.to_string(), current_dir, children: vec![([0, 0, 0, 0], Box::new(window))] }
    }
}

impl IWidget for FileChooser {
    fn handle_event(&mut self, event: &Event<'_, ()>, cursor_pos: &PhysicalPosition<f64>, signals: &mut Vec<Signal>) {
        self.get_children_mut().for_each(|(_bounds, child)| child.handle_event(event, cursor_pos, signals));
        let mut current_dir = self.current_dir.parent().unwrap().to_path_buf();
        signals.iter().for_each(|signal| match (&signal.typ, signal.source.as_str()) {
            (SignalType::Activated, "lblUp") => {
                println!("Up");
                current_dir = self.current_dir.parent().unwrap().to_path_buf();
            }
            _ => {}
        });
        let lbl_path = self.find_by_id("lblPath").unwrap();
        let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
        lbl_path.model.text = current_dir.to_str().unwrap().to_string();
        self.current_dir = current_dir;
    }

    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }

    fn get_children_mut(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter()
    }
}
