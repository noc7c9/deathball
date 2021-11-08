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
use physics::{Physics, PhysicsEvent};

mod animals;
mod buildings;
mod death_ball;

use animals::Animal;
use buildings::Building;
use death_ball::DeathBall;

const DRAW_COLLIDERS: bool = false;

pub struct Resources {
    assets: Assets,
    input: Input,
    camera: Camera,
    physics: Physics,
    deleted: Vec<GenerationalIndex>,
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
    };
    let mut physics_events: Vec<PhysicsEvent> = Vec::new();
    let mut death_ball = DeathBall::new(&mut res, Vec2::ZERO);
    let mut animals: Entities<Animal, { Animal::GROUP }> = Entities::new();
    let mut buildings: Entities<Building, { Building::GROUP }> = Entities::new();

    // Create the buildings
    for pos in [
        vec2(0., -500.),
        vec2(-344., -500.),
        vec2(344., -500.),
        vec2(0., 500.),
        vec2(-344., 500.),
        vec2(344., 500.),
    ] {
        buildings.push(|idx| Building::horizontal_fence(idx, &mut res, pos));
    }

    for pos in [
        vec2(-530., -344.),
        vec2(-530., 0.),
        vec2(-530., 344.),
        vec2(530., -344.),
        vec2(530., 0.),
        vec2(530., 344.),
    ] {
        buildings.push(|idx| Building::vertical_fence(idx, &mut res, pos));
    }

    buildings.push(|idx| Building::new(Building::VARIANTS[0], idx, &mut res, vec2(0., 0.)));

    // Create ball
    for _ in 0..10 {
        let x = rand::gen_range(-450., 450.);
        let y = rand::gen_range(-450., 450.);

        animals.push(|idx| Animal::random(idx, &mut res, vec2(x, y)));
    }

    loop {
        let delta = get_frame_time();

        // Update entities
        death_ball.update(&mut res);
        for animal in &mut animals {
            animal.update(&mut res, &death_ball);
        }
        for building in &mut buildings {
            building.update(&mut res, delta, &mut animals);
        }

        // Clear deleted entities
        for idx in res.deleted.drain(..) {
            match idx.group() {
                Animal::GROUP => animals.remove(idx),
                Building::GROUP => buildings.remove(idx),
                _ => {}
            };
        }

        // Update subsystems
        res.input.update();

        res.camera.update(&res.input);
        res.camera.enable();

        res.physics.update(&mut physics_events);
        for mut event in physics_events.drain(..) {
            // ensure the event is in a consistent order
            let (idx1, idx2) = {
                let idx1 = res.physics.get_idx(event.collider1);
                let idx2 = res.physics.get_idx(event.collider2);
                if idx2.group() < idx1.group() {
                    std::mem::swap(&mut event.collider1, &mut event.collider2);
                    (idx2, idx1)
                } else {
                    (idx1, idx2)
                }
            };

            if idx1 == DeathBall::IDX && idx2.group() == Animal::GROUP {
                let animal = &mut animals[idx2];
                animal.is_affected_by_death_ball(true);
            } else if idx1.group() == Animal::GROUP && idx2.group() == Building::GROUP {
                let animal = &mut animals[idx1];
                let building = &mut buildings[idx2];
                building.damage(animal.damage);
            }
        }

        // Draw
        clear_background(BLACK);
        death_ball.draw(&res);
        for animal in &animals {
            animal.draw(&res);
        }
        for building in &buildings {
            building.draw(&res);
        }

        if DRAW_COLLIDERS {
            res.physics.draw_colliders();
        }

        res.camera.disable();

        next_frame().await
    }
}
