use macroquad::prelude::*;

use crate::Resources;

const WIDTH: f32 = 219.;
const MARGIN: (f32, f32) = (12., 8.);
const LINE_HEIGHT: f32 = 20.;
const FONT_SIZE: u16 = 16;
const BG_COLOR: Color = Color::new(0., 0., 0., 0.733);
const FG_COLOR: Color = WHITE;

type Rows = &'static [&'static str];

pub struct TextBubble {
    position: Vec2,
    rows: Rows,
}

impl TextBubble {
    pub fn new(position: Vec2, rows: Rows) -> Self {
        TextBubble { position, rows }
    }

    pub fn draw(&self, res: &Resources) {
        let height = (LINE_HEIGHT * self.rows.len() as f32) + MARGIN.1 * 2.;

        draw_rectangle(self.position.x, self.position.y, WIDTH, height, BG_COLOR);

        for (i, text) in self.rows.iter().enumerate() {
            let x = self.position.x + MARGIN.0;
            let y = self.position.y + MARGIN.1 + LINE_HEIGHT * (i as f32 + 0.75);
            let params = TextParams {
                font: res.assets.font,
                // render at twice to make the text sharper
                font_size: FONT_SIZE * 2,
                font_scale: 0.5,
                color: FG_COLOR,
                ..Default::default()
            };
            draw_text_ex(text, x, y, params);
        }
    }
}
