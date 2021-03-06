use crate::{
    animals::Animal, audio::bgm, background::Background, buildings::Building, enemies::Enemy,
    entities::Entities, groups, objectives::Objective, text_bubbles::TextBubble, Resources,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Level {
    Test,
    Tutorial,
    Scenario1,
    Scenario2,
    Final,
}

pub use Level::*;

impl Level {
    pub fn init(&self, res: &mut Resources) -> LevelData {
        match self {
            Test => test::init(res),
            Tutorial => tutorial_scenario::init(res),
            Scenario1 => scenario_1::init(res),
            Scenario2 => scenario_2::init(res),
            Final => final_scenario::init(res),
        }
    }
}

pub struct LevelData {
    pub bgm: bgm::Track,
    pub max_score: u32,
    pub objective: Objective,
    pub background: Background,
    pub text_bubbles: Vec<TextBubble>,
    pub animals: Entities<Animal, { groups::ANIMAL }>,
    pub buildings: Entities<Building, { groups::BUILDING }>,
    pub enemies: Entities<Enemy, { groups::ENEMY }>,
}

// individual levels
mod test;

mod tutorial_scenario;

mod scenario_1;

mod scenario_2;

mod final_scenario;
