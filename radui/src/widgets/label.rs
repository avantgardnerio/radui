use std::collections::HashMap;
use std::iter::once;
use std::slice::{Iter, IterMut};

use femtovg::{Color, Paint, Path};
use uuid::Uuid;

use crate::events::{Signal, SignalType};
use crate::generated::models;
use crate::generated::models::UIComponent;
use crate::widgets::ui_component::{DrawContext, IUIComponent};

const FONT_SIZE: f32 = 24.0;
const PADDING: f32 = 2.0;

pub struct Label {
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn IUIComponent>>,
    pub listeners: HashMap<SignalType, Vec<Vec<String>>>,
}

impl IUIComponent for Label {
    fn add_event_listener(&mut self, typ: SignalType, id: Vec<String>) {
        self.listeners.entry(typ).and_modify(|v| v.push(id.clone())).or_insert_with(|| vec![id]);
    }

    fn draw(&self, ctx: &mut DrawContext) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        ctx.canvas.fill_path(&path, &Paint::color(Color::rgb(246, 245, 244)));

        let text = self.model.text_base.text.as_ref().map(|str| str.as_str()).unwrap_or("");
        let mut paint = Paint::color(Color::black());
        paint.set_font(&[ctx.font]);
        paint.set_font_size(FONT_SIZE);
        ctx.canvas.fill_text(0.0, FONT_SIZE, text, &paint).expect("Can't write");
    }

    fn update_display_list(&mut self, width: f64, height: f64) {
        self.set_actual_size(width, height);
    }

    fn measure(&mut self, ctx: &mut DrawContext) {
        let mut paint = Paint::color(Color::black());
        paint.set_font(&[ctx.font]);
        paint.set_font_size(FONT_SIZE);
        let text = self.model.text_base.text.as_ref().map(|str| str.as_str()).unwrap_or("");
        let metrics = ctx.canvas.measure_text(0.0, 0.0, text, &paint).unwrap();
        let width = metrics.width() + PADDING * 2.0;
        let height = metrics.height() + PADDING * 2.0;

        let model = self.get_model_mut();
        model.measured_width = Some(width as f64);
        model.measured_height = Some(height as f64);
        model.measured_min_width = Some(width as f64);
        model.measured_min_height = Some(height as f64);
    }

    fn handle_own_event(&mut self, path: &mut Vec<String>, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        let my_path = path.iter().cloned().chain(once(self.get_id().clone())).collect::<Vec<_>>();
        match &event.typ {
            SignalType::Click(_pos) => {
                if let Some(listeners) = self.listeners.get(&SignalType::Activated) {
                    println!("lable lisender");
                    for listener in listeners {
                        println!("dispat lable lisender");
                        let signal =
                            Signal { source: my_path.clone(), dest: listener.clone(), typ: SignalType::Activated };
                        dispatch(signal);
                    }
                }
            }
            _ => {}
        }
    }

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>> {
        self.children.iter_mut()
    }

    fn get_model(&self) -> &UIComponent {
        &self.model.text_base.ui_component
    }

    fn get_model_mut(&mut self) -> &mut UIComponent {
        &mut self.model.text_base.ui_component
    }
}

impl From<models::Label> for Box<dyn IUIComponent> {
    fn from(mut value: models::Label) -> Self {
        value.text_base.ui_component.uid = Some(Uuid::new_v4().to_string());
        let me = Label { model: value, width: 0, height: 0, children: vec![], listeners: Default::default() };
        Box::new(me)
    }
}
