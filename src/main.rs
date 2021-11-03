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
const GROUP_BOUNDARY: u8 = 2;

const DEATHBALL_IDX: GenerationalIndex = GenerationalIndex::single(0);

struct Context {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,
    physics_events: Vec<PhysicsEvent>,

    death_ball: DeathBall,
    animals: Entities<Animal, GROUP_ANIMAL>,
    boundaries: Entities<Boundary, GROUP_BOUNDARY>,
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

struct Boundary {
    handle: physics::StaticHandle,
    sprite: Sprite,
    offset: Vec2,
}

impl Boundary {
    fn horizontal_fence(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        let offset = vec2(0., -SPRITE_SIZE * 1.5);
        let size = vec2(SPRITE_SIZE * 4. * 3., SPRITE_SIZE);

        let sprite = assets.buildings.multisprite(vec2(2., 0.), vec2(3., 1.));
        let collider = physics::cuboid(size);
        let handle = physics.add_static(idx, collider, position);

        Boundary {
            sprite,
            handle,
            offset,
        }
    }

    fn vertical_fence(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        let offset = vec2(SPRITE_SIZE * 0.5, 0.);
        let size = vec2(SPRITE_SIZE, SPRITE_SIZE * 4. * 3.);

        let sprite = assets.buildings.multisprite(vec2(0., 1.), vec2(1., 3.));
        let collider = physics::cuboid(size);
        let handle = physics.add_static(idx, collider, position);

        Boundary {
            sprite,
            handle,
            offset,
        }
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

impl Animal {
    fn random(
        idx: GenerationalIndex,
        assets: &Assets,
        physics: &mut Physics,
        position: Vec2,
    ) -> Self {
        let animals = [
            ("horse", vec2(1., 0.)),
            ("duck", vec2(2., 0.)),
            ("snake", vec2(3., 0.)),
            ("mouse", vec2(4., 0.)),
            ("rabbit", vec2(5., 0.)),
            ("kuma", vec2(6., 0.)),
            ("dog", vec2(7., 0.)),
            ("cat", vec2(0., 1.)),
            ("turtle", vec2(1., 1.)),
            ("snail", vec2(2., 1.)),
            ("loaf", vec2(4., 5.)),
            ("poop", vec2(5., 5.)),
            ("rubber_ducky", vec2(6., 5.)),
        ];
        let (_, sprite) = animals[rand::gen_range(0, animals.len())];

        let sprite = assets.animals.sprite(sprite);
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
        boundaries: Entities::new(),
    };

    // Create the boundaries
    for pos in [
        vec2(0., -500.),
        vec2(-344., -500.),
        vec2(344., -500.),
        vec2(0., 500.),
        vec2(-344., 500.),
        vec2(344., 500.),
    ] {
        ctx.boundaries
            .push(|idx| Boundary::horizontal_fence(idx, &ctx.assets, &mut ctx.physics, pos));
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        ctx.boundaries
            .push(|idx| Boundary::vertical_fence(idx, &ctx.assets, &mut ctx.physics, pos));
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
        for boundary in &ctx.boundaries {
            boundary.draw(&ctx.physics);
        }

        if DRAW_COLLIDERS {
            ctx.physics.draw_colliders();
        }

        ctx.camera.disable();

        next_frame().await
    }
}
