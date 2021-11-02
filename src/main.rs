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
    animals: Vec<Animal>,
    boundaries: Vec<Boundary>,
    input: Input,
    camera: Camera,
    physics: Physics,
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
    fn horizontal_fence(ctx: &mut Context, position: Vec2) -> Self {
        let offset = vec2(0., -SPRITE_SIZE * 1.5);
        let size = vec2(SPRITE_SIZE * 4. * 3., SPRITE_SIZE);

        let sprite = ctx.assets.buildings.multisprite(vec2(2., 0.), vec2(3., 1.));
        let collider = physics::cuboid(size);
        let handle = ctx.physics.add_static(collider, position);

        Boundary {
            sprite,
            handle,
            offset,
        }
    }

    fn vertical_fence(ctx: &mut Context, position: Vec2) -> Self {
        let offset = vec2(SPRITE_SIZE * 0.5, 0.);
        let size = vec2(SPRITE_SIZE, SPRITE_SIZE * 4. * 3.);

        let sprite = ctx.assets.buildings.multisprite(vec2(0., 1.), vec2(1., 3.));
        let collider = physics::cuboid(size);
        let handle = ctx.physics.add_static(collider, position);

        Boundary {
            sprite,
            handle,
            offset,
        }
    }

    fn draw(&self, ctx: &Context) {
        let (pos, rot) = ctx.physics.get_position(self.handle);
        self.sprite.draw(pos + self.offset, rot);
    }
}

struct Animal {
    handle: physics::DynamicHandle,
    sprite: Sprite,
}

impl Animal {
    fn base(ctx: &mut Context, sprite: Sprite, position: Vec2) -> Self {
        let collider = physics::ball(SPRITE_SIZE / 2.);
        let handle = ctx.physics.add_dynamic(collider, position);
        Animal { sprite, handle }
    }

    fn random(ctx: &mut Context, position: Vec2) -> Self {
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
        animals[rand::gen_range(0, animals.len())](ctx, position)
    }

    fn horse(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(1., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn duck(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(2., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn snake(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(3., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn mouse(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(4., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn rabbit(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(5., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn kuma(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(6., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn dog(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(7., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn cat(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(0., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn turtle(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(1., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn snail(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(2., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn loaf(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(4., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn poop(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(5., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn rubber_ducky(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.assets.animals.sprite(vec2(6., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn draw(&self, ctx: &Context) {
        let (pos, rot) = ctx.physics.get_position(self.handle);
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
    let mut ctx = Context {
        assets: Assets::load().await,
        animals: vec![],
        boundaries: vec![],
        input: Input::new(),
        camera: Camera::new(),
        physics: Physics::new(),
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
        let fence = Boundary::horizontal_fence(&mut ctx, pos);
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
        let fence = Boundary::vertical_fence(&mut ctx, pos);
        ctx.boundaries.push(fence);
    }

    // Create ball
    for _ in 0..100 {
        let x = rand::gen_range(-screen_center.x + 160., screen_center.x - 160.);
        let y = rand::gen_range(-screen_center.y + 160., screen_center.y - 160.);
        let animal = Animal::random(&mut ctx, vec2(x, y));

        let vel = vec2(rand::gen_range(-500., 500.), rand::gen_range(-500., 500.));
        ctx.physics.set_linear_velocity(animal.handle, vel);

        let ang = rand::gen_range(-5., 5.);
        ctx.physics.set_angular_velocity(animal.handle, ang);

        ctx.animals.push(animal);
    }

    loop {
        ctx.input.update();

        ctx.physics.update();

        ctx.camera.update(&ctx.input);
        ctx.camera.enable();

        // Draw
        clear_background(BLACK);
        for animal in &ctx.animals {
            animal.draw(&ctx);
        }
        for boundary in &ctx.boundaries {
            boundary.draw(&ctx);
        }

        if DRAW_COLLIDERS {
            ctx.physics.draw_colliders();
        }

        ctx.camera.disable();

        next_frame().await
    }
}
