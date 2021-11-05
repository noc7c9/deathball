use macroquad::prelude::*;

mod camera;
mod entities;
mod input;
mod physics;
mod spritesheet;

use camera::Camera;
use entities::{Entities, GenerationalIndex};
use input::Input;
use physics::{Physics, PhysicsEvent};
use spritesheet::{Sprite, Spritesheet};

const DRAW_COLLIDERS: bool = false;
const SPRITE_SIZE: f32 = 32.;

// Entity Group Ids
const GROUP_ANIMAL: u8 = 1;
const GROUP_BUILDING: u8 = 2;

const DEATHBALL_IDX: GenerationalIndex = GenerationalIndex::single(0);

struct Context {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,
    physics_events: Vec<PhysicsEvent>,

    death_ball: DeathBall,
    animals: Entities<Animal, GROUP_ANIMAL>,
    buildings: Entities<Building, GROUP_BUILDING>,
}

struct Assets {
    animals: Spritesheet,
    buildings: Spritesheet,
}

impl Assets {
    async fn load() -> Self {
        let animals = load_texture("./assets/animals.png").await.unwrap();
        let buildings = load_texture("./assets/buildings.png").await.unwrap();
        Assets {
            animals: Spritesheet::new(animals, SPRITE_SIZE),
            buildings: Spritesheet::new(buildings, SPRITE_SIZE * 4.),
        }
    }
}

struct Building {
    handle: physics::StaticHandle,
    sprite: Sprite,
    offset: Vec2,
}

// name, multisprite, size, offset
type BuildingVariant = (
    &'static str,
    ((f32, f32), (f32, f32)),
    (f32, f32),
    (f32, f32),
);

impl Building {
    #[rustfmt::skip]
    const VARIANTS: [BuildingVariant; 22] = [
        ("barn",                ((1., 3.), (2., 1.)), (194., 68.), (10., -35.)),
        ("car",                 ((0., 4.), (2., 1.)), (248., 72.), (-5., -38.)),
        ("concrete_wall_h",     ((5., 0.), (3., 1.)), (350., 40.), (0., -35.)),
        ("concrete_wall_v",     ((7., 1.), (1., 3.)), (54., 322.), (-4., -28.)),
        ("down_with_horses",    ((4., 1.), (2., 1.)), (214., 64.), (-8., -7.)),
        ("feeding_trough",      ((5., 2.), (2., 1.)), (220., 48.), (2., -44.)),
        ("fence_h",             ((2., 0.), (3., 1.)), (370., 42.), (0., -51.)),
        ("fence_v",             ((0., 1.), (1., 3.)), (40., 354.), (7., -6.)),
        ("garage",              ((6., 4.), (2., 1.)), (218., 74.), (5., -19.)),
        ("hay_bale_h",          ((3., 2.), (1., 1.)), (76., 64.), (0., -11.)),
        ("hay_bale_v",          ((4., 2.), (1., 1.)), (74., 54.), (8., -35.)),
        ("horse_crossing_sign", ((2., 2.), (1., 1.)), (26., 26.), (-3., -57.)),
        ("house_1",             ((1., 1.), (2., 1.)), (160., 64.), (25., -24.)),
        ("house_2",             ((4., 4.), (2., 1.)), (206., 86.), (1., -27.)),
        ("oil_barrel",          ((6., 1.), (1., 1.)), (68., 58.), (1., -42.)),
        ("outhouse",            ((2., 4.), (1., 1.)), (64., 56.), (1., -43.)),
        ("portapotty",          ((3., 4.), (1., 1.)), (76., 58.), (0., -33.)),
        ("stable",              ((1., 0.), (1., 1.)), (102., 52.), (-4., -45.)),
        ("stable_double",       ((3., 3.), (2., 1.)), (208., 78.), (-1., -19.)),
        ("stable_wide",         ((5., 3.), (2., 1.)), (206., 72.), (2., -22.)),
        ("stop_sign",           ((3., 1.), (1., 1.)), (28., 24.), (-1., -55.)),
        ("yield_sign",          ((1., 2.), (1., 1.)), (26., 26.), (-1., -56.)),
    ];

    fn new(
        variant: BuildingVariant,
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        let (_, sprite, size, offset) = variant;

        let sprite = assets
            .buildings
            .multisprite(sprite.0.into(), sprite.1.into());
        let collider = physics::cuboid(size.into());
        let handle = physics.add_static(idx, collider, position);

        Building {
            sprite,
            handle,
            offset: offset.into(),
        }
    }

    fn horizontal_fence(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        Building::new(Building::VARIANTS[6], idx, assets, physics, position)
    }

    fn vertical_fence(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        Building::new(Building::VARIANTS[7], idx, assets, physics, position)
    }

    fn draw(&self, physics: &Physics) {
        let pos = physics.get_position(self.handle);
        let rot = physics.get_rotation(self.handle);
        self.sprite.draw(pos + self.offset, rot);
    }
}

struct DeathBall {
    handle: physics::SensorHandle,
    sprite: Sprite,
}

impl DeathBall {
    fn new(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let collider = physics::ball(SPRITE_SIZE / 2.).mass(1.).events(true, false);
        let handle = physics.add_sensor(DEATHBALL_IDX, collider, position);
        DeathBall {
            handle,
            sprite: assets.animals.sprite(vec2(7., 5.)),
        }
    }

    fn get_position(&self, physics: &mut Physics) -> Vec2 {
        physics.get_position(self.handle)
    }

    fn update(&mut self, physics: &mut Physics, input: &Input, camera: &Camera) {
        if let Some(position) = input.get_mouse_left_button_down() {
            let position = camera.screen_to_world(position);
            physics.set_position(self.handle, position);
        }
    }

    fn draw(&self, physics: &Physics) {
        let position = physics.get_position(self.handle);
        self.sprite.draw(position, 0.);
    }
}

struct Animal {
    handle: physics::DynamicHandle,
    sprite: Sprite,
    is_affected_by_death_ball: bool,
}

// name, sprite
type AnimalVariant = (&'static str, (f32, f32));

impl Animal {
    // name, sprite
    const VARIANTS: [AnimalVariant; 13] = [
        ("horse", (1., 0.)),
        ("duck", (2., 0.)),
        ("snake", (3., 0.)),
        ("mouse", (4., 0.)),
        ("rabbit", (5., 0.)),
        ("kuma", (6., 0.)),
        ("dog", (7., 0.)),
        ("cat", (0., 1.)),
        ("turtle", (1., 1.)),
        ("snail", (2., 1.)),
        ("loaf", (4., 5.)),
        ("poop", (5., 5.)),
        ("rubber_ducky", (6., 5.)),
    ];

    fn random(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        let (_, sprite) = Animal::VARIANTS[rand::gen_range(0, Animal::VARIANTS.len())];
        let sprite = assets.animals.sprite(sprite.into());
        let collider = physics::ball(SPRITE_SIZE / 2.).mass(1.);
        let handle = physics.add_dynamic(idx, collider, position);
        Animal {
            sprite,
            handle,
            is_affected_by_death_ball: false,
        }
    }

    fn update(&mut self, physics: &mut Physics, death_ball: &DeathBall) {
        const UNIT_SPEED: f32 = 10.;

        if self.is_affected_by_death_ball {
            let position = physics.get_position(self.handle);
            let impulse = (death_ball.get_position(physics) - position).normalize() * UNIT_SPEED;
            physics.apply_impulse(self.handle, impulse);
        }
    }

    fn draw(&self, physics: &Physics) {
        let pos = physics.get_position(self.handle);
        let rot = physics.get_rotation(self.handle);
        self.sprite.draw(pos, rot);
    }
}

pub fn window_config() -> Conf {
    Conf {
        window_title: "Giant Horse Deathball".to_owned(),
        window_width: 1200,
        window_height: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let assets = Assets::load().await;
    let mut physics = Physics::new();
    let death_ball = DeathBall::new(&assets, &mut physics, Vec2::ZERO);
    let mut ctx = Context {
        assets,
        input: Input::new(),
        camera: Camera::new(),
        physics,
        physics_events: Vec::new(),

        death_ball,
        animals: Entities::new(),
        buildings: Entities::new(),
    };

    // Create the buildings
    for pos in [
        vec2(0., -500.),
        vec2(-344., -500.),
        vec2(344., -500.),
        vec2(0., 500.),
        vec2(-344., 500.),
        vec2(344., 500.),
    ] {
        ctx.buildings
            .push(|idx| Building::horizontal_fence(idx, &ctx.assets, &mut ctx.physics, pos));
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        ctx.buildings
            .push(|idx| Building::vertical_fence(idx, &ctx.assets, &mut ctx.physics, pos));
    }

    // Create ball
    for _ in 0..10 {
        let x = rand::gen_range(-450., 450.);
        let y = rand::gen_range(-450., 450.);

        ctx.animals
            .push(|idx| Animal::random(idx, &ctx.assets, &mut ctx.physics, vec2(x, y)));
    }

    loop {
        // Update entities
        ctx.death_ball
            .update(&mut ctx.physics, &ctx.input, &ctx.camera);
        for animal in &mut ctx.animals {
            animal.update(&mut ctx.physics, &ctx.death_ball);
        }

        // Update subsystems
        ctx.input.update();

        ctx.camera.update(&ctx.input);
        ctx.camera.enable();

        ctx.physics.update(&mut ctx.physics_events);
        for mut event in ctx.physics_events.drain(..) {
            // ensure the event is in a consistent order
            let (idx1, idx2) = {
                let idx1 = ctx.physics.get_idx(event.collider1);
                let idx2 = ctx.physics.get_idx(event.collider2);
                if idx2.group() < idx1.group() {
                    std::mem::swap(&mut event.collider1, &mut event.collider2);
                    (idx2, idx1)
                } else {
                    (idx1, idx2)
                }
            };

            if idx1 == DEATHBALL_IDX && idx2.group() == GROUP_ANIMAL {
                let animal = &mut ctx.animals[idx2];
                animal.is_affected_by_death_ball = true;
            }
        }

        // Draw
        clear_background(BLACK);
        ctx.death_ball.draw(&ctx.physics);
        for animal in &ctx.animals {
            animal.draw(&ctx.physics);
        }
        for building in &ctx.buildings {
            building.draw(&ctx.physics);
        }

        if DRAW_COLLIDERS {
            ctx.physics.draw_colliders();
        }

        ctx.camera.disable();

        next_frame().await
    }
}
