use crate::events::Signal;
use crate::generated::models::Windows;
use crate::widgets;
use crate::widgets::label::Label;
use crate::widgets::window::Window;
use crate::widgets::{window, IWidget};
use std::env;
use std::path::PathBuf;
use yaserde::de::from_str;

pub struct FileChooser {
    pub current_dir: PathBuf,
    pub window: Option<Box<Window>>,
}

impl FileChooser {
    pub fn new() -> Self {
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

        Self { current_dir, window: Some(Box::new(window)) }
    }

    pub fn on_signal(&self, _win: &mut window::Window, _signal: &Signal) {}
}
