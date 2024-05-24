use std::env;
use std::iter::once;
use std::path::PathBuf;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use yaserde::de::from_str;

use crate::events::{Signal, SignalType};
use crate::generated::models::Windows;
use crate::widgets;
use crate::widgets::label::Label;
use crate::widgets::{IWidget, PositionedWidget};

pub struct FileChooser {
    pub name: String,
    pub id: Uuid,
    pub current_dir: PathBuf,
    pub children: Vec<PositionedWidget>,
    pub lbl_up_id: Vec<Uuid>,
}

impl FileChooser {
    pub fn new(name: &str, path: &Vec<Uuid>) -> Self {
        println!("new FC");
        let bytes = include_bytes!("../../resources/lib.xml");
        let content = String::from_utf8_lossy(bytes);
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.name == "file_chooser").unwrap();
        let window = windows.window.remove(idx);
        let mut window: widgets::window::Window = window.into();

        let current_dir = env::current_dir().unwrap();
        let lbl_path = window.find_by_name("lblPath").unwrap();
        let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
        lbl_path.model.text = current_dir.to_str().unwrap().to_string();

        let id = Uuid::new_v4();
        let my_path = path.iter().cloned().chain(once(id.clone())).collect();
        let label = window.find_by_name("lblUp").unwrap();
        label.add_event_listener(SignalType::Activated, my_path);
        let lbl_up_id = path.iter().cloned().chain(once(label.get_id().clone())).collect();

        let widget = Box::new(window);
        let bounds = [0, 0, 0, 0];
        let child = PositionedWidget { bounds, widget };
        Self { name: name.to_string(), id, current_dir, children: vec![child], lbl_up_id }
    }
}

impl IWidget for FileChooser {
    fn handle_own_event(&mut self, _path: &mut Vec<Uuid>, event: &Signal, _dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        if event.source == self.lbl_up_id {
            println!("Up");
            let current_dir = self.current_dir.parent().unwrap().to_path_buf();
            let lbl_path = self.find_by_name("lblPath").unwrap();
            let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
            lbl_path.model.text = current_dir.to_str().unwrap().to_string();
            self.current_dir = current_dir;
        }
    }

    fn get_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}
