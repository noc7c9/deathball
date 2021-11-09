use macroquad::prelude::*;

use crate::{
    animals::{Animal, Variant as AnimalVariant},
    entities::{Entities, GenerationalIndex},
    health_bar::HealthBar,
    physics,
    spritesheet::Sprite,
    Resources,
};

const SPAWN_MAX_OFFSET: f32 = 20.;

const FADE_TIME: f32 = 1.;

const HEALTH_BAR_SIZE: (f32, f32) = (128., 16.);
const HEALTH_BAR_OFFSET: (f32, f32) = (64., 92.);

pub enum Status {
    Indestructible,
    Destructible {
        health_bar: HealthBar,
        health: u8,
        max_health: u8,
    },
    Destroyed {
        fade_timer: f32,
    },
}

pub struct Building {
    idx: GenerationalIndex,
    handle: physics::StaticHandle,
    sprite: Sprite,
    offset: Vec2,
    status: Status,
    spawn_count: u8,
    guaranteed_spawns: [Option<AnimalVariant>; 3],
}

#[derive(Clone, Copy)]
pub struct Variant {
    _name: &'static str,
    sprite: ((f32, f32), (f32, f32)),
    size: (f32, f32),
    offset: (f32, f32),
    health: u8,
    spawn_count: u8,
    guaranteed_spawns: [Option<AnimalVariant>; 3],
}

impl Building {
    pub const GROUP: u8 = 2;

    pub const VARIANTS: [Variant; 22] = [
        Variant {
            _name: "barn",
            sprite: ((1., 3.), (2., 1.)),
            size: (194., 68.),
            offset: (10., -35.),
            health: 200,
            spawn_count: 3,
            guaranteed_spawns: [Some(AnimalVariant::Horse), Some(AnimalVariant::Cat), None],
        },
        Variant {
            _name: "car",
            sprite: ((0., 4.), (2., 1.)),
            size: (248., 72.),
            offset: (-5., -38.),
            health: 150,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Cat), None, None],
        },
        Variant {
            _name: "concrete_wall_h",
            sprite: ((5., 0.), (3., 1.)),
            size: (350., 40.),
            offset: (0., -35.),
            health: 0,
            spawn_count: 0,
            guaranteed_spawns: [None, None, None],
        },
        Variant {
            _name: "concrete_wall_v",
            sprite: ((7., 1.), (1., 3.)),
            size: (54., 322.),
            offset: (-4., -28.),
            health: 0,
            spawn_count: 0,
            guaranteed_spawns: [None, None, None],
        },
        Variant {
            _name: "down_with_horses",
            sprite: ((4., 1.), (2., 1.)),
            size: (214., 64.),
            offset: (-8., -7.),
            health: 100,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Horse), None, None],
        },
        Variant {
            _name: "feeding_trough",
            sprite: ((5., 2.), (2., 1.)),
            size: (220., 48.),
            offset: (2., -44.),
            health: 25,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Horse), None, None],
        },
        Variant {
            _name: "fence_h",
            sprite: ((2., 0.), (3., 1.)),
            size: (370., 42.),
            offset: (0., -51.),
            health: 0,
            spawn_count: 0,
            guaranteed_spawns: [None, None, None],
        },
        Variant {
            _name: "fence_v",
            sprite: ((0., 1.), (1., 3.)),
            size: (40., 354.),
            offset: (7., -6.),
            health: 0,
            spawn_count: 0,
            guaranteed_spawns: [None, None, None],
        },
        Variant {
            _name: "garage",
            sprite: ((6., 4.), (2., 1.)),
            size: (218., 74.),
            offset: (5., -19.),
            health: 10,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Snake), None, None],
        },
        Variant {
            _name: "hay_bale_h",
            sprite: ((3., 2.), (1., 1.)),
            size: (76., 64.),
            offset: (0., -11.),
            health: 10,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Rabbit), None, None],
        },
        Variant {
            _name: "hay_bale_v",
            sprite: ((4., 2.), (1., 1.)),
            size: (74., 54.),
            offset: (8., -35.),
            health: 10,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Cat), None, None],
        },
        Variant {
            _name: "horse_crossing_sign",
            sprite: ((2., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-3., -57.),
            health: 25,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Horse), None, None],
        },
        Variant {
            _name: "house_1",
            sprite: ((1., 1.), (2., 1.)),
            size: (160., 64.),
            offset: (25., -24.),
            health: 100,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Duck), None, None],
        },
        Variant {
            _name: "house_2",
            sprite: ((4., 4.), (2., 1.)),
            size: (206., 86.),
            offset: (1., -27.),
            health: 100,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Cat), None, None],
        },
        Variant {
            _name: "oil_barrel",
            sprite: ((6., 1.), (1., 1.)),
            size: (68., 58.),
            offset: (1., -42.),
            health: 100,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Kuma), None, None],
        },
        Variant {
            _name: "outhouse",
            sprite: ((2., 4.), (1., 1.)),
            size: (64., 56.),
            offset: (1., -43.),
            health: 10,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Poop), None, None],
        },
        Variant {
            _name: "portapotty",
            sprite: ((3., 4.), (1., 1.)),
            size: (76., 58.),
            offset: (0., -33.),
            health: 10,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Poop), None, None],
        },
        Variant {
            _name: "stable",
            sprite: ((1., 0.), (1., 1.)),
            size: (102., 52.),
            offset: (-4., -45.),
            health: 10,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Horse), None, None],
        },
        Variant {
            _name: "stable_double",
            sprite: ((3., 3.), (2., 1.)),
            size: (208., 78.),
            offset: (-1., -19.),
            health: 100,
            spawn_count: 3,
            guaranteed_spawns: [Some(AnimalVariant::Horse), Some(AnimalVariant::Horse), None],
        },
        Variant {
            _name: "stable_wide",
            sprite: ((5., 3.), (2., 1.)),
            size: (206., 72.),
            offset: (2., -22.),
            health: 100,
            spawn_count: 3,
            guaranteed_spawns: [Some(AnimalVariant::Horse), Some(AnimalVariant::Horse), None],
        },
        Variant {
            _name: "stop_sign",
            sprite: ((3., 1.), (1., 1.)),
            size: (28., 24.),
            offset: (-1., -55.),
            health: 10,
            spawn_count: 2,
            guaranteed_spawns: [Some(AnimalVariant::Dog), None, None],
        },
        Variant {
            _name: "yield_sign",
            sprite: ((1., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-1., -56.),
            health: 10,
            spawn_count: 1,
            guaranteed_spawns: [Some(AnimalVariant::Cat), None, None],
        },
    ];

    pub fn new(
        variant: Variant,
        idx: GenerationalIndex,
        res: &mut Resources,
        position: Vec2,
    ) -> Self {
        let sprite = res
            .assets
            .buildings
            .multisprite(variant.sprite.0.into(), variant.sprite.1.into());
        let collider = physics::cuboid(variant.size.into());
        let handle = res.physics.add_static(idx, collider, position);

        Building {
            idx,
            sprite,
            handle,
            offset: variant.offset.into(),
            status: if variant.health == 0 {
                Status::Indestructible
            } else {
                Status::Destructible {
                    health_bar: HealthBar::new(HEALTH_BAR_SIZE.into(), HEALTH_BAR_OFFSET.into()),
                    health: variant.health,
                    max_health: variant.health,
                }
            },
            spawn_count: variant.spawn_count,
            guaranteed_spawns: variant.guaranteed_spawns,
        }
    }

    pub fn horizontal_fence(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        Building::new(Building::VARIANTS[6], idx, res, position)
    }

    pub fn vertical_fence(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        Building::new(Building::VARIANTS[7], idx, res, position)
    }

    pub fn damage(&mut self, damage: u8) {
        if let Status::Destructible {
            ref mut health_bar,
            health,
            ..
        } = &mut self.status
        {
            health_bar.reset_fade();

            *health = health.saturating_sub(damage);
            if *health == 0 {
                self.status = Status::Destroyed {
                    fade_timer: FADE_TIME,
                };
            }
        }
    }

    pub fn update(
        &mut self,
        res: &mut Resources,
        delta: f32,
        animals: &mut Entities<Animal, { Animal::GROUP }>,
    ) {
        match self.status {
            Status::Destructible {
                ref mut health_bar, ..
            } => health_bar.update(delta),
            Status::Destroyed { ref mut fade_timer } => {
                *fade_timer -= delta;
                if *fade_timer < 0. {
                    let origin = res.physics.get_position(self.handle);

                    res.physics.remove(self.handle);
                    res.deleted.push(self.idx);

                    // spawn random animals
                    let mut remaining = self.spawn_count as i8;
                    for variant in self.guaranteed_spawns {
                        if let Some(variant) = variant {
                            remaining -= 1;
                            let position = random_position(origin, SPAWN_MAX_OFFSET);
                            animals.push(|idx| Animal::new(variant, idx, res, position));
                        } else {
                            break;
                        }
                    }
                    while remaining > 0 {
                        remaining -= 1;
                        let position = random_position(origin, SPAWN_MAX_OFFSET);
                        animals.push(|idx| Animal::random(idx, res, position));
                    }
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self, res: &Resources) {
        let position = res.physics.get_position(self.handle);
        let rotation = res.physics.get_rotation(self.handle);
        match self.status {
            Status::Indestructible => self.sprite.draw(position + self.offset, rotation),
            Status::Destructible {
                health,
                max_health,
                ref health_bar,
            } => {
                self.sprite.draw(position + self.offset, rotation);
                health_bar.draw(position, health as f32 / max_health as f32);
            }
            Status::Destroyed { fade_timer } => {
                let alpha = fade_timer / FADE_TIME;
                self.sprite
                    .draw_alpha(position + self.offset, rotation, alpha);
            }
        }
    }
}

fn random_position(center: Vec2, offset: f32) -> Vec2 {
    let dx = rand::gen_range(-offset, offset);
    let dy = rand::gen_range(-offset, offset);
    center + (dx, dy).into()
}
