use std::iter::once;
use crate::generated::models;
use crate::generated::models::WidgetChoice;
use crate::widgets::IWidget;
use itertools::{Either, Itertools};
use crate::geom::Size;

pub struct Vbox {
    pub model: models::Vbox,
    pub children: Vec<(u32, Box<dyn IWidget>)>,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Vbox {
    fn draw(
        &self,
    ) {
        println!("draw");
        for (idx, (top, child)) in self.children.iter().enumerate() {
            let bottom = self.children.get(idx + 1).map(|(t, _c)| *t).unwrap_or(self.height);
            let height = bottom - top;
            // let viewport = ctx.viewport.unwrap();
            let clip_rect = [0, *top as u32, self.width as u32, height as u32];
            // let scale_x = viewport.draw_size[0] as f64 / viewport.window_size[0];
            // let scale_y = viewport.draw_size[1] as f64 / viewport.window_size[1];
            // //println!("view={:?} trans={:?}", ctx.view, trans);
            // let clip_rect = [
            //     ((0.0 + viewport.rect[0] as f64) * scale_x) as u32,
            //     ((0.0 + viewport.rect[1] as f64) * scale_y) as u32,
            //     (self.width * scale_x) as u32,
            //     (self.height * scale_y) as u32
            // ];
            println!("child={idx} rect={clip_rect:?}");
            // let transform = ctx.transform.trans(0.0, *top);
            // let draw_state = ctx.draw_state.scissor(clip_rect);
            // let viewport = Some(Viewport {
            //     rect: [
            //         0 + viewport.rect[0],
            //         0 + viewport.rect[1],
            //         self.width as i32 + viewport.rect[2],
            //         height as i32 + viewport.rect[3],
            //     ],
            //     draw_size: viewport.draw_size,
            //     window_size: viewport.window_size
            // });
            // let clipped = Context { transform, viewport, draw_state, ..ctx.clone() };
            // child.draw(&clipped, gl, glyphs);
        }
    }

    fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) = self.children.iter().map(|(_top, c)| c.get_height()).partition_map(|h| {
            match h {
                Size::Absolute(n) => Either::Left(n),
                Size::Relative(n) => Either::Right(n),
            }
        });
        let abs_height: u32 = abs.iter().sum();
        let rel_total: u32 = rel.iter().sum();
        let remaining = height - abs_height;

        // position tops
        let mut cursor = 0;
        for (top, child) in self.children.iter_mut() {
            *top = cursor;
            println!("top={top}");
            let height = match child.get_height() {
                Size::Absolute(h) => h,
                Size::Relative(h) => h / rel_total * remaining,
            };
            cursor += height;
        }

        // layout children
        let bottoms = self.children.iter().map(|(top, _c)| *top).skip(1).chain(once(height)).collect::<Vec<_>>();
        for (idx, (top, child)) in self.children.iter_mut().enumerate() {
            let height = bottoms[idx] - *top;
            child.layout(width, height);
        }
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        todo!()
    }
}

impl From<models::Vbox> for Box<dyn IWidget> {
    fn from(mut value: models::Vbox) -> Self {
        println!("childrec={}", value.children.len());
        let children: Vec<(u32, _)> = value
            .children
            .drain(..)
            .map(|c| {
                let child: Box<dyn IWidget> = match *c.widget_choice {
                    WidgetChoice::GridView(c) => c.into(),
                    WidgetChoice::Vbox(c) => c.into(),
                    WidgetChoice::Label(c) => c.into(),
                    WidgetChoice::__Unknown__(_) => panic!("Unknown element"),
                };
                (0, child)
            })
            .collect();
        let me = Vbox { model: value, children, width: 0, height: 0 };
        Box::new(me)
    }
}
