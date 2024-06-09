use std::slice::{Iter, IterMut};

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};
use uuid::Uuid;

use crate::generated::models;
use crate::generated::models::DataGrid;
use crate::geom::Size;
use crate::widgets::{IWidget, PositionedWidget};

pub struct GridView {
    pub id: Uuid,
    pub model: DataGrid,
    pub width: u32,
    pub height: u32,
    pub children: Vec<PositionedWidget>,
}

impl IWidget for GridView {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, _font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn layout(&mut self, width: u32, height: u32, _canvas: &Canvas<OpenGl>, _font: &FontId) {
        self.width = width;
        self.height = height;
    }

    fn get_height(&self, _canvas: &Canvas<OpenGl>, _font: &FontId) -> Size {
        self.model.height.as_deref().unwrap_or("100%").parse().unwrap()
    }

    fn get_name(&self) -> Option<&str> {
        self.model.name.as_deref()
    }

    fn get_children(&self) -> Iter<'_, PositionedWidget> {
        self.children.iter()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, PositionedWidget> {
        self.children.iter_mut()
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl From<models::DataGrid> for Box<dyn IWidget> {
    fn from(value: models::DataGrid) -> Self {
        let me = GridView { id: Uuid::new_v4(), model: value, width: 0, height: 0, children: vec![] };
        Box::new(me)
    }
}
