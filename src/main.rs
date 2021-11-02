use macroquad::prelude::*;

mod camera;
mod input;
mod physics;
mod spritesheet;

use camera::Camera;
use input::Input;
use physics::Physics;
use spritesheet::{Sprite, Spritesheet};

const DRAW_COLLIDERS: bool = false;
const SPRITE_SIZE: f32 = 32.;

struct Context {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,

    death_ball: DeathBall,
    animals: Vec<Animal>,
    boundaries: Vec<Boundary>,
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
    fn base(physics: &mut Physics, sprite: Sprite, position: Vec2) -> Self {
        let collider = physics::ball(SPRITE_SIZE / 2.).mass(1.);
        let handle = physics.add_dynamic(collider, position);
        Animal { sprite, handle }
    }

    fn random(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let animals = [
            Animal::horse,
            Animal::duck,
            Animal::snake,
            Animal::mouse,
            Animal::rabbit,
            Animal::kuma,
            Animal::dog,
            Animal::cat,
            Animal::turtle,
            Animal::snail,
            Animal::loaf,
            Animal::poop,
            Animal::rubber_ducky,
        ];
        animals[rand::gen_range(0, animals.len())](assets, physics, position)
    }

    fn horse(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(1., 0.));
        Animal::base(physics, sprite, position)
    }

    fn duck(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(2., 0.));
        Animal::base(physics, sprite, position)
    }

    fn snake(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(3., 0.));
        Animal::base(physics, sprite, position)
    }

    fn mouse(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(4., 0.));
        Animal::base(physics, sprite, position)
    }

    fn rabbit(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(5., 0.));
        Animal::base(physics, sprite, position)
    }

    fn kuma(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(6., 0.));
        Animal::base(physics, sprite, position)
    }

    fn dog(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(7., 0.));
        Animal::base(physics, sprite, position)
    }

    fn cat(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(0., 1.));
        Animal::base(physics, sprite, position)
    }

    fn turtle(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(1., 1.));
        Animal::base(physics, sprite, position)
    }

    fn snail(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(2., 1.));
        Animal::base(physics, sprite, position)
    }

    fn loaf(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(4., 5.));
        Animal::base(physics, sprite, position)
    }

    fn poop(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(5., 5.));
        Animal::base(physics, sprite, position)
    }

    fn rubber_ducky(assets: &Assets, physics: &mut Physics, position: Vec2) -> Self {
        let sprite = assets.animals.sprite(vec2(6., 5.));
        Animal::base(physics, sprite, position)
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
        animals: vec![],
        boundaries: vec![],
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
        let fence = Boundary::horizontal_fence(&ctx.assets, &mut ctx.physics, pos);
        ctx.boundaries.push(fence);
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        let fence = Boundary::vertical_fence(&ctx.assets, &mut ctx.physics, pos);
        ctx.boundaries.push(fence);
    }

    // Create ball
    for _ in 0..10 {
        let x = rand::gen_range(-screen_center.x + 160., screen_center.x - 160.);
        let y = rand::gen_range(-screen_center.y + 160., screen_center.y - 160.);
        let animal = Animal::random(&ctx.assets, &mut ctx.physics, vec2(x, y));

        ctx.animals.push(animal);
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
