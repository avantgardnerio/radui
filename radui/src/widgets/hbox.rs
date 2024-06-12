use std::iter::once;
use std::slice::{Iter, IterMut};

use itertools::{Either, Itertools};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::{Components, UIComponent};
use crate::geom::{Point2d, Size};
use crate::widgets::ui_component::{DrawContext, IUIComponent};

pub struct HBox {
    pub model: models::HBox,
    pub children: Vec<Box<dyn IUIComponent>>,
}

impl IUIComponent for HBox {
    fn draw(&self, ctx: &mut DrawContext) {
        for (idx, widget) in self.children.iter().enumerate() {
            let left = widget.get_x();
            let right = self.children.get(idx + 1).map(|w| w.get_x()).unwrap_or(self.get_width());
            let width = right - left;

            ctx.canvas.save();
            ctx.canvas.translate(left as f32, 0.0);
            ctx.canvas.scissor(0.0, 0.0, width as f32, self.get_height() as f32);

            widget.draw(ctx);

            ctx.canvas.restore();
        }
    }

    fn update_display_list(&mut self, width: f64, height: f64, ctx: &DrawContext) {
        self.set_actual_size(width, height);

        // calculate size of pie
        let (abs, rel): (Vec<_>, Vec<_>) =
            self.children.iter().map(|w| w.get_width(canvas, font)).partition_map(|w| match w {
                Size::Absolute(n) => Either::Left(n as f32),
                Size::Relative(n) => Either::Right(n as f32),
            });
        let abs_width: f32 = abs.iter().sum();
        let rel_total: f32 = rel.iter().sum();
        let remaining = width as f32 - abs_width;

        // position tops
        let mut cursor = 0;
        for widget in self.children.iter_mut() {
            widget.set_x(cursor as f64);
            widget.set_y(0 as f64);
            widget.set_width(height as f64);
            let width = match widget.get_width(canvas, font) {
                Size::Absolute(w) => w,
                Size::Relative(w) => (w as f32 * remaining / rel_total) as u32,
            };
            cursor += width;
        }

        // layout children
        let rights = self.children.iter().map(|w| w.get_x()).skip(1).chain(once(width as f64)).collect::<Vec<_>>();
        for (idx, widget) in self.children.iter_mut().enumerate() {
            let left = widget.get_x();
            let width = rights[idx] - left;
            widget.set_width(width);
            // println!("Hbox bounds={:?}", widget.bounds);
            widget.update_display_list(width, width, ctx);
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let left = widget.get_x();
                    if pos.dims[0] < left as u32 {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0] - left as u32, pos.dims[1]] };
                    let ev = Signal { typ: SignalType::Click(pos), ..ev.clone() };
                    widget.handle_event(path, &ev, dispatch);
                }
            }
            _ => {}
        }
        path.pop();
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn get_model(&self) -> &UIComponent {
        &self.model.mx_box.container.ui_component
    }

    fn get_model_mut(&mut self) -> &mut UIComponent {
        &mut self.model.mx_box.container.ui_component
    }

    fn measure(&mut self, _ctx: &mut DrawContext) {
        todo!()
    }
}

impl From<models::HBox> for Box<dyn IUIComponent> {
    fn from(mut value: models::HBox) -> Self {
        println!("childrec={}", value.children.len());
        let children = value
            .children
            .drain(..)
            .map(|child| {
                let widget: Box<dyn IUIComponent> = match child {
                    Components::VBox(c) => c.into(),
                    Components::HBox(c) => c.into(),
                    Components::Label(c) => c.into(),
                    Components::DataGrid(c) => c.into(),
                    _ => unimplemented!("Not instantiable"),
                };
                widget
            })
            .collect::<Vec<_>>();
        value.mx_box.container.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = HBox { model: value, children };
        Box::new(me)
    }
}
