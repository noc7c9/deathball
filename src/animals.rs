use macroquad::prelude::*;

use crate::{
    death_ball::DeathBall, entities::GenerationalIndex, physics, spritesheet::Sprite, Resources,
};

pub struct Animal {
    handle: physics::DynamicHandle,
    sprite: Sprite,
    pub damage: u8,
    pub is_affected_by_death_ball: bool,
}

#[derive(Clone, Copy)]
pub enum Variant {
    Cat,
    Dog,
    Duck,
    Horse,
    Kuma,
    Loaf,
    Mouse,
    Poop,
    Rabbit,
    RubberDucky,
    Snail,
    Snake,
    Turtle,
}

struct VariantData {
    sprite: (f32, f32),
    damage: u8,
}

impl Variant {
    fn to_data(self) -> VariantData {
        match self {
            Variant::Cat => VariantData {
                sprite: (0., 1.),
                damage: 3,
            },
            Variant::Dog => VariantData {
                sprite: (7., 0.),
                damage: 3,
            },
            Variant::Duck => VariantData {
                sprite: (2., 0.),
                damage: 2,
            },
            Variant::Horse => VariantData {
                sprite: (1., 0.),
                damage: 1,
            },
            Variant::Kuma => VariantData {
                sprite: (6., 0.),
                damage: 4,
            },
            Variant::Loaf => VariantData {
                sprite: (4., 5.),
                damage: 5,
            },
            Variant::Mouse => VariantData {
                sprite: (4., 0.),
                damage: 2,
            },
            Variant::Poop => VariantData {
                sprite: (5., 5.),
                damage: 0,
            },
            Variant::Rabbit => VariantData {
                sprite: (5., 0.),
                damage: 1,
            },
            Variant::RubberDucky => VariantData {
                sprite: (6., 5.),
                damage: 50,
            },
            Variant::Snail => VariantData {
                sprite: (2., 1.),
                damage: 1,
            },
            Variant::Snake => VariantData {
                sprite: (3., 0.),
                damage: 3,
            },
            Variant::Turtle => VariantData {
                sprite: (1., 1.),
                damage: 2,
            },
        }
    }
}

impl Animal {
    const VARIANTS: [Variant; 13] = [
        Variant::Cat,
        Variant::Dog,
        Variant::Duck,
        Variant::Horse,
        Variant::Kuma,
        Variant::Loaf,
        Variant::Mouse,
        Variant::Poop,
        Variant::Rabbit,
        Variant::RubberDucky,
        Variant::Snail,
        Variant::Snake,
        Variant::Turtle,
    ];

    pub fn new(
        variant: Variant,
        idx: GenerationalIndex,
        res: &mut Resources,
        position: Vec2,
    ) -> Self {
        let variant = variant.to_data();
        let sprite = res.assets.animals.sprite(variant.sprite.into());
        let collider = physics::ball(16.)
            .mass(1.)
            .linear_damping(0.1)
            .contact_events();
        let handle = res.physics.add_dynamic(idx, collider, position);
        Animal {
            sprite,
            handle,
            damage: variant.damage,
            is_affected_by_death_ball: false,
        }
    }

    pub fn random(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        let variant = Animal::VARIANTS[rand::gen_range(0, Animal::VARIANTS.len())];
        Animal::new(variant, idx, res, position)
    }

    pub fn update(&mut self, res: &mut Resources, death_ball: &DeathBall) {
        const UNIT_SPEED: f32 = 10.;

        if self.is_affected_by_death_ball {
            let position = res.physics.get_position(self.handle);
            let impulse = (death_ball.get_position(res) - position).normalize() * UNIT_SPEED;
            res.physics.apply_impulse(self.handle, impulse);
        }
    }

    pub fn draw(&self, res: &Resources) {
        let pos = res.physics.get_position(self.handle);
        let rot = res.physics.get_rotation(self.handle);
        self.sprite.draw(pos, rot);
    }
}
