use radui::generated::models::Windows;
use radui::widgets;
use std::{fs};
use yaserde::de::from_str;

use radui::app::App;
use radui::events::SignalType;

fn main() {
    let filename = "resources/app.xml";
    let content = fs::read_to_string(filename).unwrap();
    let mut windows: Windows = from_str(&content).unwrap();

    let idx = windows.window.iter().position(|w| w.id == "appWindow").unwrap();
    let win = windows.window.remove(idx);
    let win: widgets::window::Window = win.into();

    App::run(win, move |win, signal| match signal.typ {
        SignalType::Activated => {
            if signal.source == "lblOpen" {
                win.file_chooser();
            }
        }
    });
}
