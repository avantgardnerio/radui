use radui::generated::models;
use radui::generated::models::Windows;
use radui::widgets;
use std::fs;
use std::num::NonZeroU32;
use yaserde::de::from_str;

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Renderer};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::{dpi::PhysicalSize, window::Window};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use resource::resource;

fn main() {
    let filename = "resources/layout.xml";
    let content = fs::read_to_string(filename).unwrap();
    let mut windows: Windows = from_str(&content).unwrap();

    let win: models::Window = windows.window.drain(..).last().expect("Expected at least 1 window");
    let mut win: widgets::window::Window = win.into();

    let event_loop = EventLoop::new();
    let (context, gl_display, window, surface) = create_window(&event_loop, win.model.title.as_str());

    let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
        .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32); // TODO: window size from model
    let font = canvas.add_font_mem(&resource!("resources/FiraSans-Regular.ttf")).expect("Cannot add font");

    let mut first = true;
    let mut mouse_position = PhysicalPosition::new(0., 0.);
    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CursorMoved { position, .. } => {
                mouse_position = position;
                window.request_redraw();
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        Event::RedrawRequested(_) => {
            // Make sure the canvas has the right size:
            let size = window.inner_size();
            canvas.set_size(size.width, size.height, window.scale_factor() as f32);

            if first {
                if let Some(c) = win.child.as_mut() {
                    c.layout(size.width, size.height)
                }
                first = false;
            }

            canvas.clear_rect(0, 0, size.width, size.height, Color::black());

            if let Some(c) = win.child.as_ref() {
                c.draw(&mut canvas, &font);
            }

            canvas.flush();
            surface.swap_buffers(&context).expect("Could not swap buffers");
        }
        _ => {}
    });
}

fn create_window(
    event_loop: &EventLoop<()>,
    title: &str,
) -> (PossiblyCurrentContext, Display, Window, Surface<WindowSurface>) {
    let window_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(1000., 600.)).with_title(title);

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) =
        display_builder.build(event_loop, template, |mut configs| configs.next().unwrap()).unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

    let mut not_current_gl_context =
        Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).unwrap() });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

    (not_current_gl_context.take().unwrap().make_current(&surface).unwrap(), gl_display, window, surface)
}
