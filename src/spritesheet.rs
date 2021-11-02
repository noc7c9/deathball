use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Spritesheet {
    texture: Texture2D,
    cell_size: f32,
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
}

impl Sprite {
    pub fn draw(&self, position: Vec2, rotation: f32) {
        draw_texture_ex(
            self.sheet.texture,
            // take the position to be the center so that it matches how rapier works
            position.x - self.size.x / 2.,
            position.y - self.size.y / 2.,
            WHITE,
            DrawTextureParams {
                source: Some(self.source),
                rotation,
                ..Default::default()
            },
        )
    }
}
