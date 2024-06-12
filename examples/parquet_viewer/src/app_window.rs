use quick_xml::de::from_str;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use radui::events::{Signal, SignalType};
use radui::generated::models::WindowedApplication;
use radui::widgets;
use radui::widgets::app_window::IAppWindow;
use radui::widgets::file_chooser::FileChooser;
use radui::widgets::IWidget;

pub struct ParquetViewerWindow {
    pub id: String,
    pub children: Vec<Box<dyn IWidget>>,
    pub popups: Vec<Box<dyn IWidget>>,
    pub lbl_open_id: String,
}

impl ParquetViewerWindow {
    pub fn new() -> Self {
        let bytes = include_bytes!("../resources/app.xml");
        let content = String::from_utf8_lossy(bytes);
        let window: WindowedApplication = from_str(&content).unwrap();

        let mut win: widgets::app_window::AppWindow = window.into();

        let id = Uuid::new_v4().to_string();
        let label = win.find_by_name("lblOpen").unwrap();
        label.add_event_listener(SignalType::Activated, vec![id.clone()]);
        let lbl_open_id = label.get_id().clone();

        let widget = Box::new(win);
        Self { id, children: vec![widget], popups: vec![], lbl_open_id }
    }
}

impl IWidget for ParquetViewerWindow {
    fn handle_own_event(
        &mut self,
        path: &mut Vec<String>,
        event: &Signal,
        _dispatch: &mut Box<dyn FnMut(Signal) + '_>,
    ) {
        println!("App event source={:?} lbl_open_id={:?}", event.source, self.lbl_open_id);
        if event.source.last() == Some(&self.lbl_open_id) && event.typ == SignalType::Activated {
            println!("showing file dialog");
            let file_chooser = FileChooser::new("fcMain", path);
            let widget = Box::new(file_chooser);
            self.popups.push(widget);
        }
    }

    fn get_name(&self) -> Option<&str> {
        Some("pvApp")
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
        todo!()
    }

    fn get_y(&self) -> f64 {
        todo!()
    }

    fn set_x(&mut self, x: f64) {
        todo!()
    }

    fn set_y(&mut self, y: f64) {
        todo!()
    }

    fn set_width(&mut self, width: f64) {
        todo!()
    }

    fn set_height(&mut self, height: f64) {
        todo!()
    }
}

impl IAppWindow for ParquetViewerWindow {
    fn get_title(&self) -> &str {
        "Parquet Viewer"
    }

    fn get_popups_mut(&mut self) -> IterMut<'_, Box<dyn IWidget>> {
        self.popups.iter_mut()
    }

    fn get_popups(&self) -> Iter<'_, Box<dyn IWidget>> {
        self.popups.iter()
    }
}
