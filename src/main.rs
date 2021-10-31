use macroquad::prelude::*;

mod camera;
mod input;
mod physics;
mod spritesheet;

use camera::Camera;
use input::Input;
use physics::Physics;
use spritesheet::{Sprite, Spritesheet};

const SPRITE_SIZE: f32 = 32.;

struct Context {
    spritesheet: Spritesheet,
    animals: Vec<Animal>,
    entities: Vec<Entity>,
    input: Input,
    camera: Camera,
    physics: Physics,
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
        let sprite = ctx.spritesheet.sprite(vec2(1., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn duck(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(2., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn snake(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(3., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn mouse(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(4., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn rabbit(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(5., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn kuma(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(6., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn dog(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(7., 0.));
        Animal::base(ctx, sprite, position)
    }

    fn cat(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(0., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn turtle(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(1., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn snail(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(2., 1.));
        Animal::base(ctx, sprite, position)
    }

    fn loaf(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(4., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn poop(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(5., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn rubber_ducky(ctx: &mut Context, position: Vec2) -> Self {
        let sprite = ctx.spritesheet.sprite(vec2(6., 5.));
        Animal::base(ctx, sprite, position)
    }

    fn draw(&self, ctx: &Context) {
        let (pos, rot) = ctx.physics.get_position(self.handle);
        self.sprite.draw(pos, rot);
    }
}

enum Entity {
    StaticRect {
        size: Vec2,
        color: Color,
        handle: physics::StaticHandle,
    },
}

impl Entity {
    fn static_rect(ctx: &mut Context, position: Vec2, size: Vec2, color: Color) -> usize {
        let collider = physics::cuboid(size);
        let handle = ctx.physics.add_static(collider, position);

        let entity = Entity::StaticRect {
            size,
            color,
            handle,
        };

        let idx = ctx.entities.len();
        ctx.entities.push(entity);
        idx
    }

    fn draw(&self, ctx: &Context) {
        use Entity::*;
        match *self {
            StaticRect {
                size,
                color,
                handle,
            } => {
                let pos = ctx.physics.get_translation(handle) - size / 2.;
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            }
        }
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
    let spritesheet = {
        let tex = load_texture("./assets/animals.png").await.unwrap();
        Spritesheet::new(tex, SPRITE_SIZE)
    };

    let mut ctx = Context {
        spritesheet,
        animals: vec![],
        entities: vec![],
        input: Input::new(),
        camera: Camera::new(),
        physics: Physics::new(),
    };

    let screen_size = vec2(screen_width(), screen_height());
    let screen_center = screen_size / 2.;

    // Create the boundaries
    {
        let s = screen_size;
        let c = screen_center;
        Entity::static_rect(&mut ctx, vec2(0., -c.y), vec2(s.x + 10., 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(0., c.y), vec2(s.x + 10., 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(-c.x, 0.), vec2(10., s.y + 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(c.x, 0.), vec2(10., s.y + 10.), BLUE);
    }

    // Create ball
    for _ in 0..100 {
        let x = rand::gen_range(-screen_center.x + 25., screen_center.x - 25.);
        let y = rand::gen_range(-screen_center.y + 25., screen_center.y - 25.);
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
        for entity in &ctx.entities {
            entity.draw(&ctx);
        }

        ctx.camera.disable();

        next_frame().await
    }
}
