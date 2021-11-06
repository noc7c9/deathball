use macroquad::prelude::*;

use crate::spritesheet::Spritesheet;

const SPRITE_SIZE: f32 = 32.;

pub struct Assets {
    pub animals: Spritesheet,
    pub buildings: Spritesheet,
}

impl Assets {
    pub async fn load() -> Self {
        let animals = load_texture("./assets/animals.png").await.unwrap();
        let buildings = load_texture("./assets/buildings.png").await.unwrap();
        Assets {
            animals: Spritesheet::new(animals, SPRITE_SIZE),
            buildings: Spritesheet::new(buildings, SPRITE_SIZE * 4.),
        }
    }
}
