use glfw_window::GlfwWindow;
use piston_window::{clear, PistonWindow, rectangle, WindowSettings};

fn main() {
    println!("hello world");

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
