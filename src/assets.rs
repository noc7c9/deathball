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
        let (animals, buildings, enemies, props, font) = futures::future::join5(
            load_texture("./assets/animals.png"),
            load_texture("./assets/buildings.png"),
            load_texture("./assets/enemies.png"),
            load_texture("./assets/props.png"),
            load_file("./assets/kenney-future.ttf"),
        )
        .await;
        Assets {
            animals: Spritesheet::new(animals.unwrap(), SPRITE_SIZE),
            buildings: Spritesheet::new(buildings.unwrap(), SPRITE_SIZE * 4.),
            enemies: Spritesheet::new(enemies.unwrap(), SPRITE_SIZE),
            props: Spritesheet::new(props.unwrap(), SPRITE_SIZE),
            font: Some(font.unwrap()),
        }
    }
}
