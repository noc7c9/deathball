use macroquad::prelude::*;

mod camera;
mod entities;
mod input;
mod physics;
mod spritesheet;

use camera::Camera;
use entities::Entities;
use input::Input;
use physics::Physics;
use spritesheet::{Sprite, Spritesheet};

const DRAW_COLLIDERS: bool = false;
const SPRITE_SIZE: f32 = 32.;

// Entity Group Ids
const GROUP_ANIMAL: u8 = 1;
const GROUP_BOUNDARY: u8 = 2;

struct Context {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,

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
    fn horizontal_fence(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let offset = vec2(0., -SPRITE_SIZE * 1.5);
        let size = vec2(SPRITE_SIZE * 4. * 3., SPRITE_SIZE);

        let sprite = assets.buildings.multisprite(vec2(2., 0.), vec2(3., 1.));
        let collider = physics::cuboid(size);
        let handle = physics.add_static(collider, position);

        Boundary {
            sprite,
            handle,
            offset,
        }
    }

    fn vertical_fence(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let offset = vec2(SPRITE_SIZE * 0.5, 0.);
        let size = vec2(SPRITE_SIZE, SPRITE_SIZE * 4. * 3.);

        let sprite = assets.buildings.multisprite(vec2(0., 1.), vec2(1., 3.));
        let collider = physics::cuboid(size);
        let handle = physics.add_static(collider, position);

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
    position: Vec2,
    sprite: Sprite,
}

impl DeathBall {
    fn new(assets: &Assets) -> Self {
        DeathBall {
            position: Vec2::ZERO,
            sprite: assets.animals.sprite(vec2(7., 5.)),
        }
    }

    fn update(&mut self, input: &Input, camera: &Camera) {
        if let Some(position) = input.get_mouse_left_button_down() {
            self.position = camera.screen_to_world(position);
        }
    }

    fn draw(&self) {
        self.sprite.draw(self.position, 0.);
    }
}

struct Animal {
    handle: physics::DynamicHandle,
    sprite: Sprite,
}

impl Animal {
    fn random(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
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
        let handle = physics.add_dynamic(collider, position);
        Animal { sprite, handle }
    }

    fn update(&mut self, physics: &mut Physics, death_ball: &DeathBall) {
        const UNIT_SPEED: f32 = 10.;

        let position = physics.get_position(self.handle);
        let impulse = (death_ball.position - position).normalize() * UNIT_SPEED;
        physics.apply_impulse(self.handle, impulse);
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
    let death_ball = DeathBall::new(&assets);
    let mut ctx = Context {
        assets,
        input: Input::new(),
        camera: Camera::new(),
        physics: Physics::new(),

        death_ball,
        animals: Entities::new(),
        boundaries: Entities::new(),
    };

    let screen_size = vec2(screen_width(), screen_height());
    let screen_center = screen_size / 2.;

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
            .push(|_| Boundary::horizontal_fence(&ctx.assets, &mut ctx.physics, pos));
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
            .push(|_| Boundary::vertical_fence(&ctx.assets, &mut ctx.physics, pos));
    }

    // Create ball
    for _ in 0..10 {
        let x = rand::gen_range(-screen_center.x + 160., screen_center.x - 160.);
        let y = rand::gen_range(-screen_center.y + 160., screen_center.y - 160.);

        ctx.animals
            .push(|_| Animal::random(&ctx.assets, &mut ctx.physics, vec2(x, y)));
    }

    loop {
        // Update entities
        ctx.death_ball.update(&ctx.input, &ctx.camera);
        for animal in &mut ctx.animals {
            animal.update(&mut ctx.physics, &ctx.death_ball);
        }

        // Update subsystems
        ctx.input.update();

        ctx.physics.update();

        ctx.camera.update(&ctx.input);
        ctx.camera.enable();

        // Draw
        clear_background(BLACK);
        ctx.death_ball.draw();
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
