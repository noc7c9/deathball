use macroquad::prelude::*;

use crate::{entities::GenerationalIndex, physics, spritesheet::Sprite, Resources};

pub struct Building {
    handle: physics::StaticHandle,
    sprite: Sprite,
    offset: Vec2,
}

#[derive(Clone, Copy)]
struct Variant {
    _name: &'static str,
    sprite: ((f32, f32), (f32, f32)),
    size: (f32, f32),
    offset: (f32, f32),
}

impl Building {
    pub const GROUP: u8 = 2;

    const VARIANTS: [Variant; 22] = [
        Variant {
            _name: "barn",
            sprite: ((1., 3.), (2., 1.)),
            size: (194., 68.),
            offset: (10., -35.),
        },
        Variant {
            _name: "car",
            sprite: ((0., 4.), (2., 1.)),
            size: (248., 72.),
            offset: (-5., -38.),
        },
        Variant {
            _name: "concrete_wall_h",
            sprite: ((5., 0.), (3., 1.)),
            size: (350., 40.),
            offset: (0., -35.),
        },
        Variant {
            _name: "concrete_wall_v",
            sprite: ((7., 1.), (1., 3.)),
            size: (54., 322.),
            offset: (-4., -28.),
        },
        Variant {
            _name: "down_with_horses",
            sprite: ((4., 1.), (2., 1.)),
            size: (214., 64.),
            offset: (-8., -7.),
        },
        Variant {
            _name: "feeding_trough",
            sprite: ((5., 2.), (2., 1.)),
            size: (220., 48.),
            offset: (2., -44.),
        },
        Variant {
            _name: "fence_h",
            sprite: ((2., 0.), (3., 1.)),
            size: (370., 42.),
            offset: (0., -51.),
        },
        Variant {
            _name: "fence_v",
            sprite: ((0., 1.), (1., 3.)),
            size: (40., 354.),
            offset: (7., -6.),
        },
        Variant {
            _name: "garage",
            sprite: ((6., 4.), (2., 1.)),
            size: (218., 74.),
            offset: (5., -19.),
        },
        Variant {
            _name: "hay_bale_h",
            sprite: ((3., 2.), (1., 1.)),
            size: (76., 64.),
            offset: (0., -11.),
        },
        Variant {
            _name: "hay_bale_v",
            sprite: ((4., 2.), (1., 1.)),
            size: (74., 54.),
            offset: (8., -35.),
        },
        Variant {
            _name: "horse_crossing_sign",
            sprite: ((2., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-3., -57.),
        },
        Variant {
            _name: "house_1",
            sprite: ((1., 1.), (2., 1.)),
            size: (160., 64.),
            offset: (25., -24.),
        },
        Variant {
            _name: "house_2",
            sprite: ((4., 4.), (2., 1.)),
            size: (206., 86.),
            offset: (1., -27.),
        },
        Variant {
            _name: "oil_barrel",
            sprite: ((6., 1.), (1., 1.)),
            size: (68., 58.),
            offset: (1., -42.),
        },
        Variant {
            _name: "outhouse",
            sprite: ((2., 4.), (1., 1.)),
            size: (64., 56.),
            offset: (1., -43.),
        },
        Variant {
            _name: "portapotty",
            sprite: ((3., 4.), (1., 1.)),
            size: (76., 58.),
            offset: (0., -33.),
        },
        Variant {
            _name: "stable",
            sprite: ((1., 0.), (1., 1.)),
            size: (102., 52.),
            offset: (-4., -45.),
        },
        Variant {
            _name: "stable_double",
            sprite: ((3., 3.), (2., 1.)),
            size: (208., 78.),
            offset: (-1., -19.),
        },
        Variant {
            _name: "stable_wide",
            sprite: ((5., 3.), (2., 1.)),
            size: (206., 72.),
            offset: (2., -22.),
        },
        Variant {
            _name: "stop_sign",
            sprite: ((3., 1.), (1., 1.)),
            size: (28., 24.),
            offset: (-1., -55.),
        },
        Variant {
            _name: "yield_sign",
            sprite: ((1., 2.), (1., 1.)),
            size: (26., 26.),
            offset: (-1., -56.),
        },
    ];

    fn new(variant: Variant, idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        let sprite = res
            .assets
            .buildings
            .multisprite(variant.sprite.0.into(), variant.sprite.1.into());
        let collider = physics::cuboid(variant.size.into());
        let handle = res.physics.add_static(idx, collider, position);

        Building {
            sprite,
            handle,
            offset: variant.offset.into(),
        }
    }

    pub fn horizontal_fence(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        Building::new(Building::VARIANTS[6], idx, res, position)
    }

    pub fn vertical_fence(idx: GenerationalIndex, res: &mut Resources, position: Vec2) -> Self {
        Building::new(Building::VARIANTS[7], idx, res, position)
    }

    pub fn draw(&self, res: &Resources) {
        let pos = res.physics.get_position(self.handle);
        let rot = res.physics.get_rotation(self.handle);
        self.sprite.draw(pos + self.offset, rot);
    }
}
