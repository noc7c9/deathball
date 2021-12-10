use macroquad::prelude::*;

use crate::audio::Sound;
use crate::spritesheet::Spritesheet;

const SPRITE_SIZE: f32 = 32.;
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
    pub fn loader() -> AssetsLoader {
        AssetsLoader::new()
    }
}

pub struct AssetsLoader {
    progress: usize,

    // textures
    animals: Option<Spritesheet>,
    buildings: Option<Spritesheet>,
    enemies: Option<Spritesheet>,
    props: Option<Spritesheet>,

    // sfx
    smack: Option<[Sound; 3]>,
    explode: Option<[Sound; 2]>,

    // music
    giant_horse_deathball: Option<Sound>,
    meadow_meadow: Option<Sound>,
    send_it: Option<Sound>,
    space: Option<Sound>,
    take_me_home: Option<Sound>,

    // misc
    icon: Option<Texture2D>,
    font: Option<Vec<u8>>,
}

pub enum Progress {
    InProgress(f32),
    Complete(Assets),
}

impl AssetsLoader {
    fn new() -> Self {
        Self {
            progress: 0,

            animals: None,
            buildings: None,
            enemies: None,
            props: None,
            smack: None,
            explode: None,
            giant_horse_deathball: None,
            meadow_meadow: None,
            send_it: None,
            space: None,
            take_me_home: None,
            icon: None,
            font: None,
        }
    }

    pub async fn progress(&mut self) -> Progress {
        use futures::future::{join, join4, join5};

        // use a simple hard coded loading process, no need to overengineer this
        match self.progress {
            0 => {
                let (icon, font) = join(
                    load_texture("./assets/img/icon.png"),
                    load_file("./assets/kenney-future.ttf"),
                )
                .await;
                self.icon = Some(icon.unwrap());
                self.font = Some(font.unwrap());
            }
            1 => {
                let (animals, buildings, enemies, props) = join4(
                    load_texture("./assets/img/animals.png"),
                    load_texture("./assets/img/buildings.png"),
                    load_texture("./assets/img/enemies.png"),
                    load_texture("./assets/img/props.png"),
                )
                .await;
                self.animals = Some(Spritesheet::new(animals.unwrap(), SPRITE_SIZE));
                self.buildings = Some(Spritesheet::new(buildings.unwrap(), SPRITE_SIZE * 4.));
                self.enemies = Some(Spritesheet::new(enemies.unwrap(), SPRITE_SIZE));
                self.props = Some(Spritesheet::new(props.unwrap(), SPRITE_SIZE));
            }
            2 => {
                let (smack1, smack2, smack3, explode1, explode2) = join5(
                    load_file("./assets/sfx/smack1.ogg"),
                    load_file("./assets/sfx/smack2.ogg"),
                    load_file("./assets/sfx/smack3.ogg"),
                    load_file("./assets/sfx/explode1.ogg"),
                    load_file("./assets/sfx/explode2.ogg"),
                )
                .await;
                self.smack = Some([
                    Sound::new(smack1.unwrap()),
                    Sound::new(smack2.unwrap()),
                    Sound::new(smack3.unwrap()),
                ]);
                self.explode = Some([Sound::new(explode1.unwrap()), Sound::new(explode2.unwrap())]);
            }
            3 => {
                let file = load_file("./assets/music/giant-horse-deathball.ogg").await;
                self.giant_horse_deathball = Some(Sound::new(file.unwrap()));
            }
            4 => {
                let file = load_file("./assets/music/meadow-meadow.ogg").await;
                self.meadow_meadow = Some(Sound::new(file.unwrap()));
            }
            5 => {
                let file = load_file("./assets/music/send-it.ogg").await;
                self.send_it = Some(Sound::new(file.unwrap()));
            }
            6 => {
                let file = load_file("./assets/music/space.ogg").await;
                self.space = Some(Sound::new(file.unwrap()));
            }
            7 => {
                let file =
                    load_file("./assets/music/take-me-home-country-roads-by-team-youwin.ogg").await;
                self.take_me_home = Some(Sound::new(file.unwrap()));
            }
            8 => {
                let assets = Assets {
                    animals: self.animals.take().unwrap(),
                    buildings: self.buildings.take().unwrap(),
                    enemies: self.enemies.take().unwrap(),
                    props: self.props.take().unwrap(),
                    smack: self.smack.take().unwrap(),
                    explode: self.explode.take().unwrap(),
                    giant_horse_deathball: self.giant_horse_deathball.take().unwrap(),
                    meadow_meadow: self.meadow_meadow.take().unwrap(),
                    send_it: self.send_it.take().unwrap(),
                    space: self.space.take().unwrap(),
                    take_me_home: self.take_me_home.take().unwrap(),
                    icon: self.icon.take().unwrap(),
                    font: self.font.take(),
                };
                return Progress::Complete(assets);
            }
            _ => unreachable!(),
        }

        self.progress += 1;
        Progress::InProgress(self.progress as f32 / 8.)
    }
}
