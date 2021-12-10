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
    pub explode: [Vec<u8>; 2],

    // misc
    pub icon: Texture2D,
    pub font: Option<Vec<u8>>,
}

impl Assets {
    pub async fn load() -> Self {
        let (
            (icon, animals, buildings),
            (enemies, props, font),
            (smack1, smack2, smack3),
            (explode1, explode2),
        ) = {
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
            let d = futures::future::join(
                load_file("./assets/sfx/explode1.ogg"),
                load_file("./assets/sfx/explode2.ogg"),
            );
            futures::future::join4(a, b, c, d)
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
            explode: [explode1.unwrap(), explode2.unwrap()],

            // misc
            icon: icon.unwrap(),
            font: Some(font.unwrap()),
        }
    }
}
