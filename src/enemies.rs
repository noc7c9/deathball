use macroquad::prelude::*;

use crate::{
    entities::GenerationalIndex, health_bar::HealthBar, physics, spritesheet::Sprite, Resources,
};

const FADE_TIME: f32 = 1.;

const HEALTH_BAR_SIZE: (f32, f32) = (32., 14.);
const HEALTH_BAR_OFFSET: (f32, f32) = (16., 36.);

pub enum Status {
    Alive {
        health_bar: HealthBar,
        health: u8,
        max_health: u8,
    },
    Dead {
        fade_timer: f32,
    },
}

pub struct Enemy {
    idx: GenerationalIndex,
    handle: physics::KinematicHandle,
    sprite: Sprite,
    status: Status,
}

#[derive(Clone, Copy)]
pub struct Variant {
    _name: &'static str,
    sprite: (f32, f32),
    health: u8,
}

impl Enemy {
    pub const GROUP: u8 = 3;

    pub const VARIANTS: [Variant; 1] = [Variant {
        _name: "tmp",
        sprite: (1., 0.),
        health: 10,
    }];

    pub fn new(
        variant: Variant,
        idx: GenerationalIndex,
        res: &mut Resources,
        position: Vec2,
    ) -> Self {
        let sprite = res.assets.enemies.sprite(variant.sprite.into());
        let collider = physics::ball(16.).mass(1.).contact_events();
        let handle = res.physics.add_kinematic(idx, collider, position);
        Enemy {
            idx,
            sprite,
            handle,
            status: Status::Alive {
                health_bar: HealthBar::new(HEALTH_BAR_SIZE.into(), HEALTH_BAR_OFFSET.into()),
                health: variant.health,
                max_health: variant.health,
            },
        }
    }

    pub fn damage(&mut self, damage: u8) {
        if let Status::Alive {
            ref mut health_bar,
            health,
            ..
        } = &mut self.status
        {
            health_bar.reset_fade();

            *health = health.saturating_sub(damage);
            if *health == 0 {
                self.status = Status::Dead {
                    fade_timer: FADE_TIME,
                };
            }
        }
    }

    pub fn update(&mut self, res: &mut Resources, delta: f32) {
        match self.status {
            Status::Alive {
                ref mut health_bar, ..
            } => health_bar.update(delta),
            Status::Dead { ref mut fade_timer } => {
                *fade_timer -= delta;
                if *fade_timer < 0. {
                    res.physics.remove(self.handle);
                    res.deleted.push(self.idx);
                }
            }
        }
    }

    pub fn draw(&self, res: &Resources) {
        let position = res.physics.get_position(self.handle);
        let rotation = res.physics.get_rotation(self.handle);
        match self.status {
            Status::Alive {
                health,
                max_health,
                ref health_bar,
            } => {
                self.sprite.draw(position, rotation);
                health_bar.draw(position, health as f32 / max_health as f32);
            }
            Status::Dead { fade_timer } => {
                let alpha = fade_timer / FADE_TIME;
                self.sprite.draw_alpha(position, rotation, alpha);
            }
        }
    }
}
