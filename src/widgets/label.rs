use crate::generated::models;
use crate::widgets::IWidget;
use crate::geom::Size;

const MENU_BACKGROUND: [f32; 4] = [246.0 / 255.0, 245.0 / 255.0, 244.0 / 255.0, 1.0];
const FONT_SIZE: u32 = 14;

pub struct Label {
    pub model: models::Label,
    pub width: u32,
    pub height: u32,
}

impl IWidget for Label {
    fn draw(
        &self,
    ) {
        // let rect = [0.0, 0.0, self.width, self.height];
        // rectangle(MENU_BACKGROUND, rect, ctx.transform, gl);
        // line_from_to(WHITE, 1.0, [0.0, 0.0], [self.width - 1.0, 0.0], ctx.transform, gl); // top
        // line_from_to(WHITE, 1.0, [0.0, 0.0], [0.0, self.height - 1.0], ctx.transform, gl); // left
        // line_from_to(GRAY, 1.0, [self.width - 1.0, self.height * 2.0], [0.0, self.height - 1.0], ctx.transform, gl); // bottom
        // line_from_to(GRAY, 1.0, [self.width - 1.0, self.height - 1.0], [self.width - 1.0, 0.0], ctx.transform, gl); // right
        // let text = Text::new_color(BLACK, FONT_SIZE);
        // text.draw_pos(&self.model.text, [2.0, FONT_SIZE as f64], glyphs, &ctx.draw_state, ctx.transform, gl).unwrap();
    }

    fn layout(&mut self, width: u32, height: u32) {
        println!("label width={width} height={height}");
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
