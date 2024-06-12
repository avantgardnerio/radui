use std::slice::{Iter, IterMut};

use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::{Components, UIComponent};
use crate::geom::Point2d;
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

    fn measure(&mut self, ctx: &mut DrawContext) {
        let mut min_width = 0.0;
        let mut min_height = 0.0;

        let mut preferred_width = 0.0;
        let mut preferred_height = 0.0;

        for child in self.get_children_mut() {
            child.measure(ctx);
        }

        for child in self.get_children() {
            let w_pref = child.get_explicit_or_measured_width();
            let h_pref = child.get_explicit_or_measured_height();

            min_width += if child.get_percent_width().is_some() { child.get_min_width() } else { w_pref };

            preferred_width += w_pref;

            let h = if child.get_percent_height().is_some() { child.get_min_height() } else { h_pref };
            min_height = if h > min_height { h } else { min_height };
            preferred_height = if h_pref > preferred_height { h_pref } else { preferred_height };
        }

        let w_padding = 8.0;
        let model = self.get_model_mut();
        model.measured_min_width = Some(min_width + w_padding);
        model.measured_min_height = Some(min_height);
        model.measured_width = Some(preferred_width + w_padding);
        model.measured_height = Some(preferred_height);
    }

    // port of BoxLayout.updateDisplayList()
    fn update_display_list(&mut self, width: f64, height: f64) {
        self.set_actual_size(width, height);

        let gap = 8.0;
        let top = 0.0;
        let mut left = 0.0;
        for obj in self.get_children_mut() {
            obj.moove(left, top);
            left += obj.get_width() + gap;
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
