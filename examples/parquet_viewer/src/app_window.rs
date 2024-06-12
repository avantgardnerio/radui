use quick_xml::de::from_str;
use std::slice::{Iter, IterMut};
use uuid::Uuid;

use radui::events::{Signal, SignalType};
use radui::generated::models::{UIComponent, WindowedApplication};
use radui::widgets;
use radui::widgets::app_window::IAppWindow;
use radui::widgets::file_chooser::FileChooser;
use radui::widgets::ui_component::{DrawContext, IUIComponent};

pub struct ParquetViewerWindow {
    pub model: UIComponent,
    pub children: Vec<Box<dyn IUIComponent>>,
    pub popups: Vec<Box<dyn IUIComponent>>,
    pub lbl_open_id: String,
}

impl ParquetViewerWindow {
    pub fn new() -> Self {
        let bytes = include_bytes!("../resources/app.xml");
        let content = String::from_utf8_lossy(bytes);
        let window: WindowedApplication = from_str(&content).unwrap();

        let mut win: widgets::app_window::AppWindow = window.into();
        let model =
            win.model.application.skinnable_container.skinnable_container_base.skinnable_component.ui_component.clone();
        let mut children = win.children.drain(..).collect::<Vec<_>>();

        let label = children[0].find_by_name("lblOpen").unwrap();
        label.add_event_listener(SignalType::Activated, vec![model.uid.clone().unwrap()]);
        let lbl_open_id = label.get_id().clone();

        Self { model, children, popups: vec![], lbl_open_id }
    }
}

impl IUIComponent for ParquetViewerWindow {
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

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn get_model(&self) -> &UIComponent {
        &self.model
    }

    fn get_model_mut(&mut self) -> &mut UIComponent {
        &mut self.model
    }

    fn measure(&mut self, _ctx: &mut DrawContext) {
        let model = self.get_model_mut();
        model.measured_width = Some(model.explicit_min_width.unwrap());
        model.measured_height = Some(model.explicit_min_height.unwrap());
        model.measured_min_width = Some(model.explicit_min_width.unwrap());
        model.measured_min_height = Some(model.explicit_min_height.unwrap());
    }
}

impl IAppWindow for ParquetViewerWindow {
    fn get_title(&self) -> &str {
        "Parquet Viewer"
    }

    fn get_popups_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.popups.iter_mut()
    }

    fn get_popups(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.popups.iter()
    }
}
