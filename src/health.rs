use macroquad::prelude::*;

const FADE_TIME: f32 = 5.;
const COLOR: Color = Color::new(0.278, 0.655, 0.149, 1.);
const BORDER_COLOR: Color = Color::new(0.192, 0.192, 0.192, 1.);
const BORDER_WIDTH: f32 = 4.;

pub struct Health {
    health: u16,
    max_health: u16,

    size: Vec2,
    offset: Vec2,
    timer: f32,
}

impl Health {
    pub fn new(max_health: u16, size: Vec2, offset: Vec2) -> Self {
        Health {
            health: max_health,
            max_health,

            size,
            offset,
            timer: 0.,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.health == 0
    }

    pub fn damage(&mut self, amount: u16) {
        if amount > 0 {
            self.timer = FADE_TIME;
            self.health = self.health.saturating_sub(amount);
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.timer -= delta;
    }

    pub fn draw(&self, mut position: Vec2) {
        let percent = self.health as f32 / self.max_health as f32;
        let alpha = self.timer / FADE_TIME;
        position -= self.offset;
        let (x, y) = position.into();
        let (w, h) = self.size.into();

        draw_rectangle_lines(
            x - BORDER_WIDTH / 2.,
            y - BORDER_WIDTH / 2.,
            w + BORDER_WIDTH,
            h + BORDER_WIDTH,
            BORDER_WIDTH,
            set_alpha(BORDER_COLOR, alpha),
        );

        draw_rectangle(x, y, w * percent, h, set_alpha(COLOR, alpha));
    }
}

fn set_alpha(mut base: Color, alpha: f32) -> Color {
    base.a = alpha;
    base
}
