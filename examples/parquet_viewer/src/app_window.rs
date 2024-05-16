use std::slice::{Iter, IterMut};

use yaserde::de::from_str;

use radui::events::{Event, Signal, SignalType};
use radui::generated::models::Windows;
use radui::geom::Bounds2d;
use radui::widgets;
use radui::widgets::file_chooser::FileChooser;
use radui::widgets::window::IWindow;
use radui::widgets::IWidget;

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
        let mut win: widgets::window::Window = win.into();

        let label = win.find_by_id("lblOpen").unwrap();
        label.add_event_listener(SignalType::Activated);

        Self { children: vec![([0, 0, 0, 0], Box::new(win))] }
    }
}

impl IWidget for AppWindow {
    fn handle_event(&mut self, event: &Event, signals: &mut Vec<Signal>) {
        self.get_children_mut().for_each(|(_bounds, child)| child.handle_event(event, signals));
        signals.iter().for_each(|signal| match (&signal.typ, signal.source.as_str()) {
            (SignalType::Activated, "lblOpen") => {
                if self.children.len() == 1 {
                    // TODO: bounds testing for clicks
                    println!("showing file dialog");
                    let file_chooser = FileChooser::new("fcMain");
                    let bounds: Bounds2d<u32> = [100, 100, 200, 200];
                    let child: ([u32; 4], Box<dyn IWidget>) = (bounds, Box::new(file_chooser));
                    self.children.push(child);
                }
            }
            _ => {}
        })
    }

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
