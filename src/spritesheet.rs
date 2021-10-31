use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Spritesheet {
    texture: Texture2D,
    size: f32,
}

impl Spritesheet {
    pub fn new(texture: Texture2D, size: f32) -> Self {
        texture.set_filter(FilterMode::Nearest);

        Spritesheet { texture, size }
    }

    pub fn sprite(&self, position: Vec2) -> Sprite {
        Sprite {
            sheet: *self,
            source: Rect::new(
                self.size * position.x,
                self.size * position.y,
                self.size,
                self.size,
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Sprite {
    sheet: Spritesheet,
    source: Rect,
}

impl Sprite {
    pub fn draw(&self, position: Vec2, rotation: f32) {
        draw_texture_ex(
            self.sheet.texture,
            // take the position to be the center so that it matches how rapier works
            position.x - self.sheet.size / 2.,
            position.y - self.sheet.size / 2.,
            WHITE,
            DrawTextureParams {
                source: Some(self.source),
                rotation,
                ..Default::default()
            },
        )
    }
}
