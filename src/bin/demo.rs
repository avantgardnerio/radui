use gfx_device_gl::{CommandBuffer, Factory, Resources};
use glfw_window::GlfwWindow;
use piston_window::color::WHITE;
use piston_window::{clear, Glyphs, PistonWindow, TextureContext, TextureSettings, Window, WindowSettings};
use radui::generated::models;
use radui::generated::models::Windows;
use radui::widgets;
use std::fs;
use yaserde::de::from_str;

fn main() {
    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let mut windows: Windows = from_str(&content).unwrap();

    let win: models::Window = windows.window.drain(..).last().expect("Expected at least 1 window");
    let mut win: widgets::window::Window = win.into();

    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new(win.model.title.as_str(), [win.model.width, win.model.height]).build().unwrap();
    let font_data: &[u8] = include_bytes!("../../resources/FiraSans-Regular.ttf");
    let factory: TextureContext<Factory, Resources, CommandBuffer> =
        TextureContext { factory: window.factory.clone(), encoder: window.factory.create_command_buffer().into() };
    let mut glyphs = Glyphs::from_bytes(font_data, factory, TextureSettings::new()).unwrap();

    let mut first = true;
    while let Some(e) = window.next() {
        let width = window.window.size().width;
        let height = window.window.size().height;
        if first {
            if let Some(c) = win.child.as_mut() {
                c.layout(width, height)
            }
            first = false;
        }

        window.draw_2d(&e, |ctx, gl, dev| {
            clear(WHITE, gl);
            if let Some(c) = win.child.as_ref() {
                c.draw(&ctx, gl, &mut glyphs)
            }
            glyphs.factory.encoder.flush(dev);
        });
    }
}
