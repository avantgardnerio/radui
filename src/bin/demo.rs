use std::fs;
use glfw_window::GlfwWindow;
use piston_window::{clear, Glyphs, PistonWindow, Text, TextureContext, TextureSettings, Transformed, WindowSettings};
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
    let font_data: &[u8] = include_bytes!("../../resources/FiraSans-Regular.ttf");
    let factory = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut glyphs = Glyphs::from_bytes(font_data, factory, TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |ctx, gl, dev| {
            clear([0.5, 0.5, 0.5, 1.0], gl);
            match win.child.widget_choice.as_ref() {
                WidgetChoice::Grid(_grid) => {
                    // TODO: grid
                }
                WidgetChoice::Label(lbl) => {
                    lbl.hi();
                    let font_size = 24;
                    let transform = ctx.transform.trans(0.0, font_size as f64);
                    let white = [1.0, 1.0, 1.0, 1.0];
                    Text::new_color(white, font_size)
                        .draw(&lbl.text, &mut glyphs, &ctx.draw_state, transform, gl)
                        .unwrap();
                    glyphs.factory.encoder.flush(dev);
                }
                WidgetChoice::__Unknown__(_) => {}
            }
        });
    }
}
