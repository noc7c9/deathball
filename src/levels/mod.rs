use crate::{
    animals::Animal, background::Background, buildings::Building, enemies::Enemy,
    entities::Entities, groups,
};

pub struct Level {
    pub background: Background,
    pub animals: Entities<Animal, { groups::ANIMAL }>,
    pub buildings: Entities<Building, { groups::BUILDING }>,
    pub enemies: Entities<Enemy, { groups::ENEMY }>,
}

// individual levels
pub mod test;