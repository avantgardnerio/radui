use std::env;
use std::num::NonZeroU32;

use crate::events;
use crate::events::{Signal, SignalType};
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color};
use glutin::surface::Surface;
use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use resource::resource;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::WindowBuilder;
use winit::{dpi::PhysicalSize, window::Window};

use crate::geom::Point2d;
use crate::widgets::app_window::IAppWindow;
use crate::widgets::ui_component::DrawContext;

pub struct App {}

impl App {
    pub fn run<W: IAppWindow>(mut win: W) {
        let event_loop = EventLoopBuilder::<Signal>::with_user_event().build();
        let (context, gl_display, window, surface) = create_window(&event_loop, win.get_title());

        let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Cannot create renderer");

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
        canvas.set_size(1000, 600, window.scale_factor() as f32); // TODO: window size from model
        let font = canvas.add_font_mem(&resource!("resources/FiraSans-Regular.ttf")).expect("Cannot add font");
        let mut ctx = DrawContext { canvas, font };

        let mut first = true;
        let mut mouse_pos: Point2d<u32> = Point2d { dims: [0, 0] };
        let mut events = vec![];
        let proxy = event_loop.create_proxy();
        event_loop.run(move |ev, _target, control_flow| {
            match &ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_pos = Point2d { dims: [position.x as u32, position.y as u32] };
                        window.request_redraw();
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { state, .. } => match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {
                            let ev = Signal { source: vec![], dest: vec![], typ: SignalType::Click(mouse_pos.clone()) };
                            {
                                let ar = &mut events;
                                let mut dispatch: Box<dyn FnMut(Signal) + '_> = Box::new(move |ev: Signal| {
                                    ar.push(ev);
                                });
                                win.handle_event(&mut vec![], &ev, &mut dispatch);
                            }

                            let size = window.inner_size();
                            win.update_display_list(size.width as f64, size.height as f64);
                            window.request_redraw();
                        }
                    },
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    println!("redraw");
                    // Make sure the canvas has the right size:
                    let size = window.inner_size();
                    ctx.canvas.set_size(size.width, size.height, window.scale_factor() as f32);

                    if first {
                        win.validate_size(true, &mut ctx);
                        win.update_display_list(size.width as f64, size.height as f64);
                        first = false;
                    }

                    ctx.canvas.clear_rect(0, 0, size.width, size.height, Color::black());

                    win.draw(&mut ctx);

                    ctx.canvas.flush();
                    surface.swap_buffers(&context).expect("Could not swap buffers");
                }
                Event::UserEvent(ev) => {
                    println!("handling custom events");
                    let ar = &mut events;
                    let mut dispatch: Box<dyn FnMut(Signal) + '_> = Box::new(move |ev: events::Signal| {
                        ar.push(ev);
                    });
                    win.handle_event(&mut vec![], &ev, &mut dispatch);
                    window.request_redraw();
                }
                _ => {}
            }

            for event in events.drain(..) {
                println!("Dispatching signal");
                proxy.send_event(event).unwrap();
            }
        });
    }
}

fn create_window<T>(
    event_loop: &EventLoop<T>,
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
