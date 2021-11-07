use macroquad::prelude::*;

use crate::{entities::GenerationalIndex, physics, spritesheet::Sprite, Resources};

const FADE_TIME: f32 = 1.;

const HEALTH_BAR_FADE_TIME: f32 = 5.;
const HEALTH_BAR_COLOR: Color = Color::new(0.278, 0.655, 0.149, 1.);
const HEALTH_BAR_SIZE: (f32, f32) = (128., 16.);
const HEALTH_BAR_OFFSET: (f32, f32) = (64., 92.);
const HEALTH_BAR_BORDER_COLOR: Color = Color::new(0.192, 0.192, 0.192, 1.);
const HEALTH_BAR_BORDER_WIDTH: f32 = 4.;

#[derive(Debug)]
pub enum Status {
    Indestructible,
    Destructible {
        health_bar_fade_timer: f32,
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
}

#[derive(Clone, Copy)]
pub struct Variant {
    _name: &'static str,
    sprite: ((f32, f32), (f32, f32)),
    size: (f32, f32),
    offset: (f32, f32),
    health: u8,
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
        },
        Variant {
            _name: "car",
            sprite: ((0., 4.), (2., 1.)),
            size: (248., 72.),
            offset: (-5., -38.),
            health: 150,
        },
        Variant {
            _name: "concrete_wall_h",
            sprite: ((5., 0.), (3., 1.)),
            size: (350., 40.),
            offset: (0., -35.),
            health: 0,
        },
        Variant {
            _name: "concrete_wall_v",
            sprite: ((7., 1.), (1., 3.)),
            size: (54., 322.),
            offset: (-4., -28.),
            health: 0,
        },
        Variant {
            _name: "down_with_horses",
            sprite: ((4., 1.), (2., 1.)),
            size: (214., 64.),
            offset: (-8., -7.),
            health: 100,
        },
        Variant {
            _name: "feeding_trough",
            sprite: ((5., 2.), (2., 1.)),
            size: (220., 48.),
            offset: (2., -44.),
            health: 25,
        },
        Variant {
            _name: "fence_h",
            sprite: ((2., 0.), (3., 1.)),
            size: (370., 42.),
            offset: (0., -51.),
            health: 0,
        },
        Variant {
            _name: "fence_v",
            sprite: ((0., 1.), (1., 3.)),
            size: (40., 354.),
            offset: (7., -6.),
            health: 0,
        },
        Variant {
            _name: "garage",
            sprite: ((6., 4.), (2., 1.)),
            size: (218., 74.),
            offset: (5., -19.),
            health: 10,
        },
        Variant {
            _name: "hay_bale_h",
            sprite: ((3., 2.), (1., 1.)),
            size: (76., 64.),
            offset: (0., -11.),
            health: 10,
        },
        Variant {
            _name: "hay_bale_v",
            sprite: ((4., 2.), (1., 1.)),
            size: (74., 54.),
            offset: (8., -35.),
            health: 10,
        },
        Variant {
            _name: "horse_crossing_sign",
            sprite: ((2., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-3., -57.),
            health: 25,
        },
        Variant {
            _name: "house_1",
            sprite: ((1., 1.), (2., 1.)),
            size: (160., 64.),
            offset: (25., -24.),
            health: 100,
        },
        Variant {
            _name: "house_2",
            sprite: ((4., 4.), (2., 1.)),
            size: (206., 86.),
            offset: (1., -27.),
            health: 100,
        },
        Variant {
            _name: "oil_barrel",
            sprite: ((6., 1.), (1., 1.)),
            size: (68., 58.),
            offset: (1., -42.),
            health: 100,
        },
        Variant {
            _name: "outhouse",
            sprite: ((2., 4.), (1., 1.)),
            size: (64., 56.),
            offset: (1., -43.),
            health: 10,
        },
        Variant {
            _name: "portapotty",
            sprite: ((3., 4.), (1., 1.)),
            size: (76., 58.),
            offset: (0., -33.),
            health: 10,
        },
        Variant {
            _name: "stable",
            sprite: ((1., 0.), (1., 1.)),
            size: (102., 52.),
            offset: (-4., -45.),
            health: 10,
        },
        Variant {
            _name: "stable_double",
            sprite: ((3., 3.), (2., 1.)),
            size: (208., 78.),
            offset: (-1., -19.),
            health: 100,
        },
        Variant {
            _name: "stable_wide",
            sprite: ((5., 3.), (2., 1.)),
            size: (206., 72.),
            offset: (2., -22.),
            health: 100,
        },
        Variant {
            _name: "stop_sign",
            sprite: ((3., 1.), (1., 1.)),
            size: (28., 24.),
            offset: (-1., -55.),
            health: 10,
        },
        Variant {
            _name: "yield_sign",
            sprite: ((1., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-1., -56.),
            health: 10,
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
                    health_bar_fade_timer: 0.0,
                    health: variant.health,
                    max_health: variant.health,
                }
            },
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
            ref mut health_bar_fade_timer,
            health,
            ..
        } = &mut self.status
        {
            *health_bar_fade_timer = HEALTH_BAR_FADE_TIME;

            *health = health.saturating_sub(damage);
            if *health == 0 {
                self.status = Status::Destroyed {
                    fade_timer: FADE_TIME,
                };
            }
        }
    }

    pub fn update(&mut self, res: &mut Resources, delta: f32) {
        match self.status {
            Status::Destructible {
                ref mut health_bar_fade_timer,
                ..
            } => {
                *health_bar_fade_timer -= delta;
            }
            Status::Destroyed { ref mut fade_timer } => {
                *fade_timer -= delta;
                if *fade_timer < 0. {
                    res.physics.remove(self.handle);
                    res.deleted.push(self.idx);
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
                health_bar_fade_timer,
            } => {
                self.sprite.draw(position + self.offset, rotation);

                // draw health bar
                let percent = health as f32 / max_health as f32;
                let alpha = health_bar_fade_timer / HEALTH_BAR_FADE_TIME;
                draw_health_bar(position, percent, alpha)
            }
            Status::Destroyed { fade_timer } => {
                let alpha = fade_timer / FADE_TIME;
                self.sprite
                    .draw_alpha(position + self.offset, rotation, alpha);
            }
        }
    }
}

fn draw_health_bar(position: Vec2, percent: f32, alpha: f32) {
    let x = position.x - HEALTH_BAR_OFFSET.0;
    let y = position.y - HEALTH_BAR_OFFSET.1;
    let (w, h) = HEALTH_BAR_SIZE;
    let bw = HEALTH_BAR_BORDER_WIDTH;

    draw_rectangle_lines(
        x - bw / 2.,
        y - bw / 2.,
        w + bw,
        h + bw,
        bw,
        set_alpha(HEALTH_BAR_BORDER_COLOR, alpha),
    );

    draw_rectangle(x, y, w * percent, h, set_alpha(HEALTH_BAR_COLOR, alpha));
}

fn set_alpha(mut base: Color, alpha: f32) -> Color {
    base.a = alpha;
    base
}
