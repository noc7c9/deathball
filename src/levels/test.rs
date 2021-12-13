use macroquad::prelude::*;

use crate::{
    animals::Animal,
    audio::bgm,
    background::{Background, Prop::*},
    buildings::{Building, Variant::*},
    enemies::{Enemy, Variant::*},
    entities::Entities,
    levels::LevelData,
    objectives::Objective,
    Resources,
};

pub fn init(res: &mut Resources) -> LevelData {
    let objective = Objective::none();

    let background = Background::new(
        Color::new(59. / 255., 99. / 255., 38. / 255., 1.),
        vec2(0., 0.),
        vec![
            ((2, 2), Grass1),
            ((3, 2), Grass2),
            ((4, 2), Grass3),
            ((2, 3), Gravel1),
            ((3, 3), Gravel2),
            ((4, 3), Gravel3),
            ((2, 5), FlowerWhite),
            ((3, 5), FlowerYellow),
            ((4, 5), FlowerRed),
            ((5, 5), FlowerBlack),
            ((6, 5), Eggplant),
            ((8, 2), Mud),
            ((8, 3), Hay),
        ],
    );

    let mut animals = Entities::new();
    let mut buildings = Entities::new();
    let mut enemies = Entities::new();

    for pos in [
        vec2(0., -500.),
        vec2(-344., -500.),
        vec2(344., -500.),
        vec2(0., 500.),
        vec2(-344., 500.),
        vec2(344., 500.),
    ] {
        buildings.push(|idx| Building::new(FenceH, idx, res, pos));
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        buildings.push(|idx| Building::new(FenceV, idx, res, pos));
    }

    buildings.push(|idx| Building::new(Barn, idx, res, vec2(0., 0.)));

    enemies.push(|idx| Enemy::new(Demon, idx, res, vec2(0., 100.)));

    for _ in 0..10 {
        let x = rand::gen_range(-450., 450.);
        let y = rand::gen_range(-450., 450.);

        animals.push(|idx| Animal::random(idx, res, vec2(x, y)));
    }

    LevelData {
        bgm: bgm::MeadowMeadow,
        max_score: 0,
        objective,
        background,
        animals,
        buildings,
        enemies,
        text_bubbles: vec![],
    }
}
