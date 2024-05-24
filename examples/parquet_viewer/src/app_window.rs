use std::slice::{Iter, IterMut};
use uuid::Uuid;

use yaserde::de::from_str;

use radui::events::{Signal, SignalType};
use radui::generated::models::Windows;
use radui::geom::Bounds2d;
use radui::widgets;
use radui::widgets::file_chooser::FileChooser;
use radui::widgets::window::IWindow;
use radui::widgets::{IWidget, PositionedWidget};

pub struct ParquetViewerWindow {
    pub id: Uuid,
    pub children: Vec<PositionedWidget>,
    pub popups: Vec<PositionedWidget>,
    pub lbl_open_id: Uuid,
}

impl ParquetViewerWindow {
    pub fn new() -> Self {
        let bytes = include_bytes!("../resources/app.xml");
        let content = String::from_utf8_lossy(bytes);
        let mut windows: Windows = from_str(&content).unwrap();

        let idx = windows.window.iter().position(|w| w.name == "appWindow").unwrap();
        let win = windows.window.remove(idx);
        let mut win: widgets::window::Window = win.into();

        let id = Uuid::new_v4();
        let label = win.find_by_name("lblOpen").unwrap();
        label.add_event_listener(SignalType::Activated, vec![id.clone()]);
        let lbl_open_id = label.get_id().clone();

        let bounds = [0, 0, 0, 0];
        let widget = Box::new(win);
        let child = PositionedWidget { bounds, widget };
        Self { id, children: vec![child], popups: vec![], lbl_open_id }
    }
}

impl IWidget for ParquetViewerWindow {
    fn handle_own_event(&mut self, path: &mut Vec<Uuid>, event: &Signal, _dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        println!("App event source={:?} lbl_open_id={:?}", event.source, self.lbl_open_id);
        if event.source.last() == Some(&self.lbl_open_id) && event.typ == SignalType::Activated {
            println!("showing file dialog");
            let file_chooser = FileChooser::new("fcMain", path);
            let bounds: Bounds2d<u32> = [100, 100, 200, 200];
            let widget = Box::new(file_chooser);
            let child = PositionedWidget { bounds, widget };
            self.popups.push(child);
        }
    }

    fn get_name(&self) -> Option<&str> {
        Some("pvApp")
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
