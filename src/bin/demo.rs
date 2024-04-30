use std::fs;
use glfw_window::GlfwWindow;
use piston_window::{clear, PistonWindow, rectangle, WindowSettings};
use yaserde::de::{Deserializer, from_str};
use radui::generated::models::{Pointbuffer, Pointbuffers};

fn main() {
    println!("hello world");

    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let buffers: Pointbuffers = from_str(&content).unwrap();
    println!("buffers={buffers:#?}");
    
    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new("title", [640, 480])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}
