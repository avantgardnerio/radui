use std::fs;
use gfx_device_gl::{Factory, Resources, CommandBuffer};
use glfw_window::GlfwWindow;
use piston_window::{clear, Glyphs, PistonWindow, Text, TextureContext, TextureSettings, Transformed, WindowSettings};
use piston_window::color::WHITE;
use yaserde::de::{from_str};
use radui::generated::models::{WidgetChoice, Windows};
use radui::widgets::IWidget;

fn main() {
    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let windows: Windows = from_str(&content).unwrap();

    let win = windows.window.get(0).expect("Expected at least 1 window");

    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new(win.title.as_str(), [win.width, win.height])
            .build().unwrap();
    let font_data: &[u8] = include_bytes!("../../resources/FiraSans-Regular.ttf");
    let factory: TextureContext<Factory, Resources, CommandBuffer> = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut glyphs = Glyphs::from_bytes(font_data, factory, TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |ctx, gl, dev| {
            clear(WHITE, gl);
            match win.child.widget_choice.as_ref() {
                WidgetChoice::Vbox(vbox) => vbox.draw(&ctx, gl, &mut glyphs),
                WidgetChoice::Label(lbl) => lbl.draw(&ctx, gl, &mut glyphs),
                WidgetChoice::__Unknown__(_) => {}
                WidgetChoice::GridView(_) => {}
            }
            glyphs.factory.encoder.flush(dev);
        });
    }
}
