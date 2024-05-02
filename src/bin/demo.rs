use as_any::AsAny;
use radui::generated::models::Windows;
use radui::widgets;
use std::any::Any;
use std::{env, fs};
use yaserde::de::from_str;

use radui::app::App;
use radui::events::SignalType;
use radui::geom::Bounds2d;
use radui::widgets::label::Label;
use radui::widgets::IWidget;

fn main() {
    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let mut windows: Windows = from_str(&content).unwrap();

    let idx = windows.window.iter().position(|w| w.id == "file_chooser").unwrap();
    let file_chooser = windows.window.remove(idx);
    let file_chooser: widgets::window::Window = file_chooser.into();
    let mut file_chooser = Some(file_chooser);

    let idx = windows.window.iter().position(|w| w.id == "appWindow").unwrap();
    let win = windows.window.remove(idx);
    let win: widgets::window::Window = win.into();

    App::run(win, move |win, signal| match signal.typ {
        SignalType::Activated => {
            if signal.source == "lblOpen" {
                if let Some(mut file_chooser) = file_chooser.take() {
                    println!("showing dialog");
                    let mut lbl_path = file_chooser.find_by_id("lblPath").unwrap();
                    let mut lbl_path = lbl_path.as_mut().as_any_mut().downcast_mut::<Label>().unwrap();
                    lbl_path.model.text = env::current_dir().unwrap().to_str().unwrap().to_string();
                    let bounds: Bounds2d<u32> = [100, 100, 200, 200];
                    let child: ([u32; 4], Box<dyn IWidget>) = (bounds, Box::new(file_chooser));
                    win.children.push(child);
                    win.layout(win.width, win.height);
                }
            }
        }
    });
}
