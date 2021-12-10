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

    // music
    pub giant_horse_deathball: Vec<u8>,
    pub meadow_meadow: Vec<u8>,
    pub send_it: Vec<u8>,
    pub space: Vec<u8>,
    pub take_me_home: Vec<u8>,

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
            (giant_horse_deathball, meadow_meadow, send_it, space, take_me_home),
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
            let e = futures::future::join5(
                load_file("./assets/music/giant-horse-deathball.ogg"),
                load_file("./assets/music/meadow-meadow.ogg"),
                load_file("./assets/music/send-it.ogg"),
                load_file("./assets/music/space.ogg"),
                load_file("./assets/music/take-me-home-country-roads-by-team-youwin.ogg"),
            );
            futures::future::join5(a, b, c, d, e)
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

            // music
            giant_horse_deathball: giant_horse_deathball.unwrap(),
            meadow_meadow: meadow_meadow.unwrap(),
            send_it: send_it.unwrap(),
            space: space.unwrap(),
            take_me_home: take_me_home.unwrap(),

            // misc
            icon: icon.unwrap(),
            font: Some(font.unwrap()),
        }
    }
}
