use crate::generated::models;
use crate::geom::Size;
use crate::widgets::IWidget;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, FontId, Paint, Path};

const FONT_SIZE: f32 = 22.0;

pub struct Label {
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Label {
    fn draw(&self, canvas: &mut Canvas<OpenGl>, font: &FontId) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::rgb(246, 245, 244)));

        let mut paint = Paint::color(Color::black());
        paint.set_font(&[*font]);
        paint.set_font_size(FONT_SIZE);
        canvas.fill_text(0.0, FONT_SIZE, self.model.text.as_str(), &paint).expect("Can't write");
    }

    fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn get_width(&self) -> Size {
        todo!()
    }

    fn get_height(&self) -> Size {
        Size::Absolute(FONT_SIZE as u32 * 2)
    }
}

impl From<models::Label> for Box<dyn IWidget> {
    fn from(value: models::Label) -> Self {
        let me = Label { model: value, width: 0, height: 0 };
        Box::new(me)
    }
}
