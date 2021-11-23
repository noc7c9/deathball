use macroquad::prelude::*;

use crate::spritesheet::Spritesheet;

const SPRITE_SIZE: f32 = 32.;

pub struct Assets {
    pub animals: Spritesheet,
    pub buildings: Spritesheet,
    pub enemies: Spritesheet,
    pub props: Spritesheet,
    pub font: Option<Vec<u8>>,
}

impl Assets {
    pub async fn load() -> Self {
        let animals = load_texture("./assets/animals.png").await.unwrap();
        let buildings = load_texture("./assets/buildings.png").await.unwrap();
        let enemies = load_texture("./assets/enemies.png").await.unwrap();
        let props = load_texture("./assets/props.png").await.unwrap();
        let font = load_file("./assets/kenney-future.ttf").await.unwrap();
        Assets {
            animals: Spritesheet::new(animals, SPRITE_SIZE),
            buildings: Spritesheet::new(buildings, SPRITE_SIZE * 4.),
            enemies: Spritesheet::new(enemies, SPRITE_SIZE),
            props: Spritesheet::new(props, SPRITE_SIZE),
            font: Some(font),
        }
    }
}
