use macroquad::prelude::*;

use crate::{
    death_ball::DeathBall,
    entities::GenerationalIndex,
    physics::{self},
    spritesheet::Sprite,
    Resources,
};

pub struct Animal {
    handle: physics::DynamicHandle,
    sprite: Sprite,
    is_affected_by_death_ball: bool,
}

#[derive(Clone, Copy)]
struct Variant {
    _name: &'static str,
    sprite: (f32, f32),
}

impl Animal {
    pub const GROUP: u8 = 1;

    const VARIANTS: [Variant; 13] = [
        Variant {
            _name: "horse",
            sprite: (1., 0.),
        },
        Variant {
            _name: "duck",
            sprite: (2., 0.),
        },
        Variant {
            _name: "snake",
            sprite: (3., 0.),
        },
        Variant {
            _name: "mouse",
            sprite: (4., 0.),
        },
        Variant {
            _name: "rabbit",
            sprite: (5., 0.),
        },
        Variant {
            _name: "kuma",
            sprite: (6., 0.),
        },
        Variant {
            _name: "dog",
            sprite: (7., 0.),
        },
        Variant {
            _name: "cat",
            sprite: (0., 1.),
        },
        Variant {
            _name: "turtle",
            sprite: (1., 1.),
        },
        Variant {
            _name: "snail",
            sprite: (2., 1.),
        },
        Variant {
            _name: "loaf",
            sprite: (4., 5.),
        },
        Variant {
            _name: "poop",
            sprite: (5., 5.),
        },
        Variant {
            _name: "rubber_ducky",
            sprite: (6., 5.),
        },
    ];

    pub fn random(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        let variant = Animal::VARIANTS[rand::gen_range(0, Animal::VARIANTS.len())];
        let sprite = res.assets.animals.sprite(variant.sprite.into());
        let collider = physics::ball(16.).mass(1.);
        let handle = res.physics.add_dynamic(idx, collider, position);
        Animal {
            sprite,
            handle,
            is_affected_by_death_ball: false,
        }
    }

    pub fn is_affected_by_death_ball(&mut self, value: bool) {
        self.is_affected_by_death_ball = value;
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
