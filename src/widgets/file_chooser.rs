use std::{env, fs};
use std::path::PathBuf;
use yaserde::de::from_str;
use crate::events::Signal;
use crate::generated::models::Windows;
use crate::widgets;
use crate::widgets::label::Label;
use crate::widgets::{IWidget, window};
use crate::widgets::window::Window;

pub struct FileChooser {
    pub current_dir: PathBuf,
    pub window: Option<Box<Window>>,
}

impl FileChooser {
    pub fn new() -> Self {
        let filename = "resources/lib.xml";
        let content = fs::read_to_string(filename).unwrap();
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.id == "file_chooser").unwrap();
        let window = windows.window.remove(idx);
        let mut window: widgets::window::Window = window.into();

        let current_dir = env::current_dir().unwrap();
        let lbl_path = window.find_by_id("lblPath").unwrap();
        let lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
        lbl_path.model.text = current_dir.to_str().unwrap().to_string();

        Self {
            current_dir,
            window: Some(Box::new(window)),
        }
    }

    pub fn on_signal(&self, win: &mut window::Window, signal: &Signal) {

    }
}