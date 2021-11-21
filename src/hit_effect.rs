use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::{entities::GenerationalIndex, Resources};

// effect constants
const NUM_PARTICLES: usize = 10;
const EFFECT_LIFETIME: f32 = 2.0;
const SPAWN_RADIUS: f32 = 13.39;

// particle constants
const LIFETIME: (f32, f32) = (0.25, 1.0);
const SIZE: (f32, f32) = (1.0, 37.71);
const LINEAR_SPEED: f32 = 0.65;
const INITIAL_ORBIT: (f32, f32) = (-0.785, 0.785);
const ANGULAR_SPEED: f32 = -0.05;

pub struct HitEffect {
    idx: GenerationalIndex,
    origin: Vec2,
    life_timer: f32,
    particles: [Particle; NUM_PARTICLES],
}

#[derive(Clone, Copy)]
struct Particle {
    origin: Vec2,
    size: f32,

    life_timer: f32,
    offset: f32,
    orbit: f32,
}

impl Particle {
    fn new() -> Self {
        let origin = {
            let angle = rand::gen_range(0., 1.0) * 2. * PI;
            let radius = SPAWN_RADIUS * rand::gen_range(0., 1.0f32).sqrt();
            vec2_from_polar(radius, angle)
        };
        let size = rand::gen_range(SIZE.0, SIZE.1);
        let life_timer = rand::gen_range(LIFETIME.0, LIFETIME.1);
        let orbit = rand::gen_range(INITIAL_ORBIT.0, INITIAL_ORBIT.1);

        Particle {
            origin,
            size,
            life_timer,
            offset: 0.0,
            orbit,
        }
    }
}

impl HitEffect {
    pub fn new(idx: GenerationalIndex, origin: Vec2) -> Self {
        HitEffect {
            idx,
            origin,
            life_timer: EFFECT_LIFETIME,
            particles: [Particle {
                life_timer: 0.,
                ..Particle::new()
            }; NUM_PARTICLES],
        }
    }

    pub fn update(&mut self, res: &mut Resources) {
        self.life_timer -= res.delta;
        if self.life_timer < 0. {
            res.deleted.push(self.idx);
            return;
        }

        for particle in &mut self.particles {
            particle.life_timer -= res.delta;
            if particle.life_timer < 0. {
                // reset any particles that have finished
                *particle = Particle::new();
            }

            particle.offset += LINEAR_SPEED;
            particle.orbit += ANGULAR_SPEED;
        }
    }

    pub fn draw(&self) {
        for particle in self.particles {
            let w = particle.size;
            let h = particle.size;

            let pos = {
                let origin = self.origin + particle.origin;
                origin + vec2_from_polar(particle.offset, particle.orbit)
            };

            draw_rectangle(pos.x - w / 2., pos.y - h / 2., w, h, WHITE);
        }
    }
}

fn vec2_from_polar(radius: f32, angle: f32) -> Vec2 {
    radius * vec2(angle.cos(), angle.sin())
}
