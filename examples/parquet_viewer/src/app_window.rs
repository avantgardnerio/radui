use radui::generated::models::Windows;
use radui::geom::Bounds2d;
use radui::widgets;
use radui::widgets::window::{IWindow, Window};
use radui::widgets::IWidget;
use std::slice::{Iter, IterMut};
use yaserde::de::from_str;

pub struct AppWindow {
    pub children: Vec<(Bounds2d<u32>, Box<dyn IWidget>)>,
}

impl AppWindow {
    pub fn new() -> Self {
        let bytes = include_bytes!("../resources/app.xml");
        let content = String::from_utf8_lossy(bytes);
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.id == "appWindow").unwrap();
        let win = windows.window.remove(idx);
        let win: widgets::window::Window = win.into();

        Self { children: vec![([0, 0, 0, 0], Box::new(win))] }
    }
}

impl IWidget for AppWindow {
    fn get_id(&self) -> Option<&str> {
        Some("pvApp")
    }

    fn get_children_mut(&mut self) -> IterMut<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, (Bounds2d<u32>, Box<dyn IWidget>)> {
        self.children.iter()
    }
}

impl IWindow for AppWindow {
    fn get_title(&self) -> &str {
        "Parquet Viewer"
    }
}
