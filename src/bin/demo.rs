use std::fs;
use glfw_window::GlfwWindow;
use piston_window::{clear, PistonWindow, rectangle, WindowSettings};
use piston_window::ellipse::circle;
use yaserde::de::{from_str};
use radui::generated::models::{WidgetChoice, Windows};

fn main() {
    println!("hello world");

    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let windows: Windows = from_str(&content).unwrap();

    let win = windows.window.get(0).expect("Expected at least 1 window");

    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new(win.title.as_str(), [win.width, win.height])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            match &win.child.widget_choice {
                WidgetChoice::Box(b) => {
                    rectangle([1.0, 0.0, 0.0, 1.0], // red
                              [b.x, b.y, b.width, b.height],
                              c.transform, g);
                }
                WidgetChoice::Circle(c) => {
                    circle(c.x, c.y, c.radius);
                }
                WidgetChoice::__Unknown__(_) => {}
            }
            circle(50.0, 50.0, 100.0);
        });
    }
}
