use macroquad::prelude::*;

use crate::audio::Sound;
use crate::spritesheet::Spritesheet;

const SPRITE_SIZE: f32 = 32.;

// bytes are stored as Options so that they can be dropped once loaded by the appropriate system
pub struct Assets {
    // textures
    pub animals: Spritesheet,
    pub buildings: Spritesheet,
    pub enemies: Spritesheet,
    pub props: Spritesheet,

    // sfx
    pub smack: [Sound; 3],
    pub explode: [Sound; 2],

    // music
    pub giant_horse_deathball: Sound,
    pub meadow_meadow: Sound,
    pub send_it: Sound,
    pub space: Sound,
    pub take_me_home: Sound,

    // misc
    pub icon: Texture2D,
    pub font: Option<Vec<u8>>,
}

impl Assets {
    pub async fn load() -> Self {
        use futures::future::join4;
        let (
            (icon, animals, buildings, enemies),
            (props, font, smack1, smack2),
            (smack3, explode1, explode2, giant_horse_deathball),
            (meadow_meadow, send_it, space, take_me_home),
        ) = join4(
            join4(
                load_texture("./assets/img/icon.png"),
                load_texture("./assets/img/animals.png"),
                load_texture("./assets/img/buildings.png"),
                load_texture("./assets/img/enemies.png"),
            ),
            join4(
                load_texture("./assets/img/props.png"),
                load_file("./assets/kenney-future.ttf"),
                load_file("./assets/sfx/smack1.ogg"),
                load_file("./assets/sfx/smack2.ogg"),
            ),
            join4(
                load_file("./assets/sfx/smack3.ogg"),
                load_file("./assets/sfx/explode1.ogg"),
                load_file("./assets/sfx/explode2.ogg"),
                load_file("./assets/music/giant-horse-deathball.ogg"),
            ),
            join4(
                load_file("./assets/music/meadow-meadow.ogg"),
                load_file("./assets/music/send-it.ogg"),
                load_file("./assets/music/space.ogg"),
                load_file("./assets/music/take-me-home-country-roads-by-team-youwin.ogg"),
            ),
        )
        .await;

        Assets {
            // textures
            animals: Spritesheet::new(animals.unwrap(), SPRITE_SIZE),
            buildings: Spritesheet::new(buildings.unwrap(), SPRITE_SIZE * 4.),
            enemies: Spritesheet::new(enemies.unwrap(), SPRITE_SIZE),
            props: Spritesheet::new(props.unwrap(), SPRITE_SIZE),

            // sfx
            smack: [
                Sound::new(smack1.unwrap()),
                Sound::new(smack2.unwrap()),
                Sound::new(smack3.unwrap()),
            ],
            explode: [Sound::new(explode1.unwrap()), Sound::new(explode2.unwrap())],

            // music
            giant_horse_deathball: Sound::new(giant_horse_deathball.unwrap()),
            meadow_meadow: Sound::new(meadow_meadow.unwrap()),
            send_it: Sound::new(send_it.unwrap()),
            space: Sound::new(space.unwrap()),
            take_me_home: Sound::new(take_me_home.unwrap()),

            // misc
            icon: icon.unwrap(),
            font: Some(font.unwrap()),
        }
    }
}
