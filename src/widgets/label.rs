use femtovg::{Canvas, Color, FontId, Paint, Path};
use femtovg::renderer::OpenGl;
use crate::generated::models;
use crate::widgets::IWidget;
use crate::geom::Size;

const FONT_SIZE: f32 = 14.0;

pub struct Label {
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Label {
    fn draw(
        &self,
        canvas: &mut Canvas<OpenGl>,
        font: &FontId,
    ) {
        let mut path = Path::new();
        path.rect(0.0, 0.0, self.width as f32, self.height as f32);
        canvas.fill_path(&path, &Paint::color(Color::rgb(246, 245, 244)));

        // line_from_to(WHITE, 1.0, [0.0, 0.0], [self.width - 1.0, 0.0], ctx.transform, gl); // top
        // line_from_to(WHITE, 1.0, [0.0, 0.0], [0.0, self.height - 1.0], ctx.transform, gl); // left
        // line_from_to(GRAY, 1.0, [self.width - 1.0, self.height * 2.0], [0.0, self.height - 1.0], ctx.transform, gl); // bottom
        // line_from_to(GRAY, 1.0, [self.width - 1.0, self.height - 1.0], [self.width - 1.0, 0.0], ctx.transform, gl); // right
        // let text = Text::new_color(BLACK, FONT_SIZE);
        // text.draw_pos(&self.model.text, [2.0, FONT_SIZE as f64], glyphs, &ctx.draw_state, ctx.transform, gl).unwrap();

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
