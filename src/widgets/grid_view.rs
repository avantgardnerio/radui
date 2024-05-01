use femtovg::{Canvas, Color, FontId, Paint, Path};
use femtovg::renderer::OpenGl;
use crate::generated::models;
use crate::widgets;
use crate::widgets::IWidget;
use crate::geom::Size;

pub struct GridView {
    pub model: models::GridView,
    pub width: u32,
    pub height: u32,
}

impl IWidget for GridView {
    fn draw(
        &self,
        canvas: &mut Canvas<OpenGl>,
        font: &FontId,
    ) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::white()));
    }

    fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        self.model.height.as_ref().map(|h| h.as_str()).unwrap_or("100%").parse().unwrap()
    }
}

impl From<models::GridView> for Box<dyn IWidget> {
    fn from(value: models::GridView) -> Self {
        let me = widgets::grid_view::GridView { model: value, width: 0, height: 0 };
        Box::new(me)
    }
}
