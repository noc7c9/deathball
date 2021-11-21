use macroquad::prelude::*;

mod assets;
mod camera;
mod entities;
mod input;
mod physics;
mod spritesheet;

use assets::Assets;
use camera::Camera;
use entities::{Entities, GenerationalIndex};
use input::Input;
use physics::{Physics, PhysicsEvent, PhysicsEventKind};

mod levels;

use levels::Level;

mod animals;
mod background;
mod buildings;
mod death_ball;
mod enemies;
mod health;
mod hit_effect;

use death_ball::DeathBall;

const DRAW_COLLIDERS: bool = false;

pub mod groups {
    pub const DEATH_BALL: super::GenerationalIndex = super::GenerationalIndex::single(0);

    pub const ANIMAL: u8 = 1;

    pub const BUILDING: u8 = 2;

    pub const ENEMY: u8 = 3;
    pub const ENEMY_ATTACK: u8 = 4;

    pub const HIT_EFFECT: u8 = 5;
}

pub struct Resources {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,
    deleted: Vec<GenerationalIndex>,
    delta: f32,
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
    let mut res = Resources {
        assets: Assets::load().await,
        input: Input::new(),
        camera: Camera::new(),
        physics: Physics::new(),
        deleted: Vec::new(),
        delta: 0.,
    };
    let mut physics_events: Vec<PhysicsEvent> = Vec::new();

    let Level {
        background,
        mut animals,
        mut buildings,
        mut enemies,
    } = levels::test::init(&mut res);
    let mut death_ball = DeathBall::new(&mut res, Vec2::ZERO);
    let mut hit_effects = Entities::<hit_effect::HitEffect, { groups::HIT_EFFECT }>::new();

    loop {
        res.delta = get_frame_time();

        // Update entities
        death_ball.update(&mut res);
        for animal in &mut animals {
            animal.update(&mut res, &death_ball);
        }
        for building in &mut buildings {
            building.update(&mut res, &mut animals);
        }
        for enemy in &mut enemies {
            enemy.update(&mut res);
        }
        for hit_effect in &mut hit_effects {
            hit_effect.update(&mut res);
        }

        // Clear deleted entities
        for idx in res.deleted.drain(..) {
            match idx.group() {
                groups::ANIMAL => animals.remove(idx),
                groups::BUILDING => buildings.remove(idx),
                groups::ENEMY => enemies.remove(idx),
                groups::HIT_EFFECT => hit_effects.remove(idx),
                _ => {}
            };
        }

        // Update subsystems
        res.input.update();

        res.camera.update(&res.input);
        res.camera.enable();

        res.physics.update(&mut physics_events);
        for event in physics_events.drain(..) {
            let idx1 = res.physics.get_idx(event.collider1);
            let idx2 = res.physics.get_idx(event.collider2);

            // DeathBall with Animal
            if idx1 == groups::DEATH_BALL && idx2.group() == groups::ANIMAL {
                let animal = &mut animals[idx2];
                animal.is_affected_by_death_ball(true);
            }
            // Animal with Building
            else if idx1.group() == groups::ANIMAL && idx2.group() == groups::BUILDING {
                let animal = &mut animals[idx1];
                let building = &mut buildings[idx2];
                building.damage(animal.damage);
            }
            // Animal with Enemy
            else if idx1.group() == groups::ANIMAL && idx2.group() == groups::ENEMY {
                let animal = &mut animals[idx1];
                let enemy = &mut enemies[idx2];

                match event.kind {
                    // will only happen for detection range sensor
                    PhysicsEventKind::IntersectStart => enemy.add_nearby(event.collider1),
                    PhysicsEventKind::IntersectEnd => enemy.remove_nearby(event.collider1),

                    // will only happen for collision body
                    PhysicsEventKind::ContactStart => enemy.damage(animal.damage),
                    _ => {}
                }
            }
            // Animal with Enemy Attacks
            else if idx1.group() == groups::ANIMAL && idx2.group() == groups::ENEMY_ATTACK {
                let animal = &mut animals[idx1];
                let animal_handle = event.collider1;
                let enemy = &enemies[idx2.with_group(groups::ENEMY)];
                let enemy_handle = event.collider2;

                let animal_pos = res.physics.get_position(animal_handle);
                let enemy_pos = res.physics.get_position(enemy_handle);
                let direction = (animal_pos - enemy_pos).normalize_or_zero();

                animal.is_affected_by_death_ball(false);
                res.physics
                    .apply_impulse(animal_handle, direction * enemy.attack_impulse);
            }
        }

        // Draw
        background.draw();
        death_ball.draw(&res);
        for animal in &animals {
            animal.draw(&res);
        }
        for enemy in &enemies {
            enemy.draw(&res);
        }
        for building in &buildings {
            building.draw(&res);
        }
        for hit_effect in &hit_effects {
            hit_effect.draw();
        }

        if DRAW_COLLIDERS {
            res.physics.draw_colliders();
        }

        res.camera.disable();

        next_frame().await
    }
}
