use std::slice::{Iter, IterMut};

use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::{Components, UIComponent};
use crate::geom::Point2d;
use crate::widgets::ui_component::{DrawContext, IUIComponent};

pub struct Vbox {
    pub model: models::VBox,
    pub children: Vec<Box<dyn IUIComponent>>,
}

impl IUIComponent for Vbox {
    fn draw(&self, ctx: &mut DrawContext) {
        for (idx, widget) in self.children.iter().enumerate() {
            let top = widget.get_y();
            let bottom = self.children.get(idx + 1).map(|w| w.get_y()).unwrap_or(self.get_height());
            let height = bottom - top;

            ctx.canvas.save();
            ctx.canvas.translate(0.0, top as f32);
            ctx.canvas.scissor(0.0, 0.0, self.get_width() as f32, height as f32);

            widget.draw(ctx);

            ctx.canvas.restore();
        }
    }

    fn update_display_list(&mut self, width: f64, height: f64) {
        self.set_actual_size(width, height);

        let gap = 8.0;
        let left = 0.0;
        let mut top = 0.0;
        for obj in self.get_children_mut() {
            obj.moove(left, top);
            top += obj.get_height() + gap;
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, ev: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());
        match &ev.typ {
            SignalType::Click(pos) => {
                for widget in self.children.iter_mut().rev() {
                    let top = widget.get_y();
                    if pos.dims[1] < top as u32 {
                        continue;
                    }
                    let pos = Point2d { dims: [pos.dims[0], pos.dims[1] - top as u32] };
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

impl From<models::VBox> for Box<dyn IUIComponent> {
    fn from(mut value: models::VBox) -> Self {
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
        let me = Vbox { model: value, children };
        Box::new(me)
    }
}
