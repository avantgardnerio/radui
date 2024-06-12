use quick_xml::de::from_str;
use std::env;
use std::iter::once;
use std::path::PathBuf;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models::TitleWindow;
use crate::widgets::label::Label;
use crate::widgets::modal::Modal;
use crate::widgets::IWidget;

pub struct FileChooser {
    pub name: String,
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub current_dir: PathBuf,
    pub children: Vec<Box<dyn IWidget>>,
    pub lbl_up_id: Vec<String>,
}

impl FileChooser {
    pub fn new(name: &str, path: &Vec<String>) -> Self {
        println!("new FC");
        let bytes = include_bytes!("../../resources/lib.xml");
        let content = String::from_utf8_lossy(bytes);
        let window: TitleWindow = from_str(&content).unwrap();

        let mut window: Modal = window.into();

        let current_dir = env::current_dir().unwrap();
        let lbl_path = window.find_by_name("lblPath").unwrap();
        let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
        lbl_path.model.text_base.text = current_dir.to_str().map(|str| str.to_string());

        let id = Uuid::new_v4().to_string();
        let my_path = path.iter().cloned().chain(once(id.clone())).collect();
        let label = window.find_by_name("lblUp").unwrap();
        label.add_event_listener(SignalType::Activated, my_path);
        let lbl_up_id = path.iter().cloned().chain(once(label.get_id().clone())).collect();

        let widget = Box::new(window);
        Self {
            name: name.to_string(),
            id,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            current_dir,
            children: vec![widget],
            lbl_up_id,
        }
    }
}

impl IWidget for FileChooser {
    fn handle_own_event(
        &mut self,
        _path: &mut Vec<String>,
        event: &Signal,
        _dispatch: &mut Box<dyn FnMut(Signal) + '_>,
    ) {
        if event.source == self.lbl_up_id {
            println!("Up");
            let current_dir = self.current_dir.parent().unwrap().to_path_buf();
            let lbl_path = self.find_by_name("lblPath").unwrap();
            let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
            lbl_path.model.text_base.text = current_dir.to_str().map(|str| str.to_string());
            self.current_dir = current_dir;
        }
    }

    fn get_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IWidget>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IWidget>> {
        self.children.iter()
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}
