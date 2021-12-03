use macroquad::prelude::*;

use crate::{
    animals::Animal,
    background::{Background, Prop},
    buildings::Building,
    enemies::Enemy,
    entities::Entities,
    levels::LevelData,
    objectives::Objective,
    Resources,
};

pub fn init(res: &mut Resources) -> LevelData {
    let objective = Objective::none();

    let background =
        Background::builder(Color::new(59. / 255., 99. / 255., 38. / 255., 1.), (26, 20))
            .set_prop((2, 2), Prop::Grass1)
            .set_prop((3, 2), Prop::Grass2)
            .set_prop((4, 2), Prop::Grass3)
            .set_prop((2, 3), Prop::Gravel1)
            .set_prop((3, 3), Prop::Gravel2)
            .set_prop((4, 3), Prop::Gravel3)
            .set_prop((2, 5), Prop::FlowerWhite)
            .set_prop((3, 5), Prop::FlowerYellow)
            .set_prop((4, 5), Prop::FlowerRed)
            .set_prop((5, 5), Prop::FlowerBlack)
            .set_prop((6, 5), Prop::Eggplant)
            .set_prop((8, 2), Prop::Mud)
            .set_prop((8, 3), Prop::Hay)
            .build(res);

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
        buildings.push(|idx| Building::horizontal_fence(idx, res, pos));
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        buildings.push(|idx| Building::vertical_fence(idx, res, pos));
    }

    buildings.push(|idx| Building::new(Building::VARIANTS[0], idx, res, vec2(0., 0.)));

    enemies.push(|idx| Enemy::new(Enemy::VARIANTS[0], idx, res, vec2(0., 100.)));

    for _ in 0..10 {
        let x = rand::gen_range(-450., 450.);
        let y = rand::gen_range(-450., 450.);

        animals.push(|idx| Animal::random(idx, res, vec2(x, y)));
    }

    LevelData {
        objective,
        background,
        animals,
        buildings,
        enemies,
    }
}
