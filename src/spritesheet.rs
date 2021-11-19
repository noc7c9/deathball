use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Spritesheet {
    texture: Texture2D,
    pub cell_size: f32,
}

impl Spritesheet {
    pub fn new(texture: Texture2D, cell_size: f32) -> Self {
        texture.set_filter(FilterMode::Nearest);

        Spritesheet { texture, cell_size }
    }

    pub fn sprite(&self, position: Vec2) -> Sprite {
        self.multisprite(position, vec2(1., 1.))
    }

    pub fn multisprite(&self, position: Vec2, size: Vec2) -> Sprite {
        let size = self.cell_size * size;
        Sprite {
            sheet: *self,
            size,
            scale: 1.,
            source: Rect::new(
                self.cell_size * position.x,
                self.cell_size * position.y,
                size.x,
                size.y,
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Sprite {
    sheet: Spritesheet,
    size: Vec2,
    source: Rect,
    scale: f32,
}

impl Sprite {
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        self.draw_alpha(position, rotation, 1.)
    }

    pub fn draw_alpha(&self, position: Vec2, rotation: f32, alpha: f32) {
        let mut color = WHITE;
        color.a = alpha;
        self.draw_tint(position, rotation, color)
    }

    pub fn draw_tint(&self, position: Vec2, rotation: f32, color: Color) {
        let size = self.size * self.scale;
        draw_texture_ex(
            self.sheet.texture,
            // take the position to be the center so that it matches how rapier works
            position.x - size.x / 2.,
            position.y - size.y / 2.,
            color,
            DrawTextureParams {
                dest_size: Some(size),
                source: Some(self.source),
                rotation,
                ..Default::default()
            },
        )
    }

    pub fn draw_top_right(&self, position: Vec2) {
        draw_texture_ex(
            self.sheet.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.source),
                ..Default::default()
            },
        )
    }
}
