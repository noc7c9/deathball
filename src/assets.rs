use macroquad::prelude::*;

use crate::spritesheet::Spritesheet;

const SPRITE_SIZE: f32 = 32.;

pub struct Assets {
    // textures
    pub animals: Spritesheet,
    pub buildings: Spritesheet,
    pub enemies: Spritesheet,
    pub props: Spritesheet,

    // sfx
    pub smack: [Vec<u8>; 3],

    // misc
    pub icon: Texture2D,
    pub font: Option<Vec<u8>>,
}

impl Assets {
    pub async fn load() -> Self {
        let ((icon, animals, buildings), (enemies, props, font), (smack1, smack2, smack3)) = {
            let a = futures::future::join3(
                load_texture("./assets/img/icon.png"),
                load_texture("./assets/img/animals.png"),
                load_texture("./assets/img/buildings.png"),
            );
            let b = futures::future::join3(
                load_texture("./assets/img/enemies.png"),
                load_texture("./assets/img/props.png"),
                load_file("./assets/kenney-future.ttf"),
            );
            let c = futures::future::join3(
                load_file("./assets/sfx/smack1.ogg"),
                load_file("./assets/sfx/smack2.ogg"),
                load_file("./assets/sfx/smack3.ogg"),
            );
            futures::future::join3(a, b, c)
        }
        .await;
        Assets {
            // textures
            animals: Spritesheet::new(animals.unwrap(), SPRITE_SIZE),
            buildings: Spritesheet::new(buildings.unwrap(), SPRITE_SIZE * 4.),
            enemies: Spritesheet::new(enemies.unwrap(), SPRITE_SIZE),
            props: Spritesheet::new(props.unwrap(), SPRITE_SIZE),

            // sfx
            smack: [smack1.unwrap(), smack2.unwrap(), smack3.unwrap()],

            // misc
            icon: icon.unwrap(),
            font: Some(font.unwrap()),
        }
    }
}
