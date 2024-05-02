use radui::generated::models::Windows;
use radui::widgets;
use std::fs;
use yaserde::de::from_str;

use radui::app::App;
use radui::events::SignalType;
use radui::geom::Bounds2d;
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
                if let Some(file_chooser) = file_chooser.take() {
                    println!("showing dialog");
                    let bounds: Bounds2d<u32> = [20, 20, 400, 300];
                    let child: ([u32; 4], Box<dyn IWidget>) = (bounds, Box::new(file_chooser));
                    win.children.push(child);
                }
            }
        }
    });
}
