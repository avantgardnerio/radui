use std::slice::{Iter, IterMut};

use yaserde::de::from_str;

use radui::events::{Signal, SignalType};
use radui::generated::models::Windows;
use radui::geom::Bounds2d;
use radui::widgets;
use radui::widgets::file_chooser::FileChooser;
use radui::widgets::window::IWindow;
use radui::widgets::{IWidget, PositionedWidget};

pub struct ParquetViewerWindow {
    pub children: Vec<PositionedWidget>,
    pub popups: Vec<PositionedWidget>,
}

impl ParquetViewerWindow {
    pub fn new() -> Self {
        let bytes = include_bytes!("../resources/app.xml");
        let content = String::from_utf8_lossy(bytes);
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.id == "appWindow").unwrap();
        let win = windows.window.remove(idx);
        let mut win: widgets::window::Window = win.into();

        let label = win.find_by_id("lblOpen").unwrap();
        label.add_event_listener(SignalType::Activated);

        let bounds = [0, 0, 0, 0];
        let widget = Box::new(win);
        let child = PositionedWidget { bounds, widget };
        Self { children: vec![child], popups: vec![] }
    }
}

impl IWidget for ParquetViewerWindow {
    fn handle_event(&mut self, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        self.get_children_mut().for_each(|widget| widget.widget.handle_event(event, dispatch));

        match (&event.typ, event.source.as_str()) {
            (SignalType::Activated, "lblOpen") => {
                println!("showing file dialog");
                let file_chooser = FileChooser::new("fcMain");
                let bounds: Bounds2d<u32> = [100, 100, 200, 200];
                let widget = Box::new(file_chooser);
                let child = PositionedWidget { bounds, widget };
                self.popups.push(child);
            }
            _ => {}
        }
    }

    fn get_id(&self) -> Option<&str> {
        Some("pvApp")
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }
}

impl IWindow for ParquetViewerWindow {
    fn get_title(&self) -> &str {
        "Parquet Viewer"
    }

    fn get_popups_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.popups.iter_mut()
    }

    fn get_popups(&self) -> Iter<'_, PositionedWidget> {
        self.popups.iter()
    }
}
