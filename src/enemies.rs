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
        health: u16,
        max_health: u16,
        speed: f32,
    },
    Dead {
        fade_timer: f32,
    },
}

pub struct Enemy {
    idx: GenerationalIndex,
    handle: physics::DynamicHandle,
    sensor_handle: physics::SensorHandle,
    nearby_animals: Vec<physics::Handle>,
    sprite: Sprite,
    status: Status,
}

#[derive(Clone, Copy)]
pub struct Variant {
    _name: &'static str,
    sprite: (f32, f32),
    health: u16,
    speed: f32,
    detection_range: f32,
}

impl Enemy {
    pub const GROUP: u8 = 3;

    pub const VARIANTS: [Variant; 6] = [
        Variant {
            _name: "demon",
            sprite: (5., 0.),
            health: 200,
            speed: 75.,
            detection_range: 600.,
        },
        Variant {
            _name: "demon_boss",
            sprite: (5., 0.),
            health: 400,
            speed: 50.,
            detection_range: 6000.,
        },
        Variant {
            _name: "farmer",
            sprite: (1., 0.),
            health: 10,
            speed: 50.,
            detection_range: 300.,
        },
        Variant {
            _name: "police",
            sprite: (2., 0.),
            health: 20,
            speed: 50.,
            detection_range: 400.,
        },
        Variant {
            _name: "snowman",
            sprite: (4., 0.),
            health: 25,
            speed: 25.,
            detection_range: 600.,
        },
        Variant {
            _name: "soldier",
            sprite: (3., 0.),
            health: 50,
            speed: 60.,
            detection_range: 400.,
        },
    ];

    pub fn new(
        variant: Variant,
        idx: GenerationalIndex,
        res: &mut Resources,
        position: Vec2,
    ) -> Self {
        let sprite = res.assets.enemies.sprite(variant.sprite.into());

        // add a dynamic body with very large mass so that we mimic a kinematic body that
        // can't be moved by collisions from animals
        // but will not intersect static bodies
        let collider = physics::ball(16.).mass(1_000_000_000.).contact_events();
        let handle = res.physics.add_dynamic(idx, collider, position);

        let collider = physics::ball(variant.detection_range).intersection_events();
        let sensor_handle = res.physics.add_sensor(idx, collider, position);

        Enemy {
            idx,
            sprite,
            handle,
            sensor_handle,
            nearby_animals: Vec::new(),
            status: Status::Alive {
                health_bar: HealthBar::new(HEALTH_BAR_SIZE.into(), HEALTH_BAR_OFFSET.into()),
                health: variant.health,
                max_health: variant.health,
                speed: variant.speed,
            },
        }
    }

    pub fn add_nearby(&mut self, animal: physics::Handle) {
        self.nearby_animals.push(animal);
    }
    pub fn remove_nearby(&mut self, animal: physics::Handle) {
        self.nearby_animals.retain(|a| *a != animal);
    }

    pub fn damage(&mut self, damage: u8) {
        if let Status::Alive {
            ref mut health_bar,
            health,
            ..
        } = &mut self.status
        {
            health_bar.reset_fade();

            *health = health.saturating_sub(damage as u16);
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
                speed,
                ref mut health_bar,
                ..
            } => {
                health_bar.update(delta);

                let position = res.physics.get_position(self.handle);

                // ensure sensor collider moves with the enemy
                res.physics.set_position(self.sensor_handle, position);

                // move towards the first nearby animal
                let velocity = if let Some(first) = self.nearby_animals.first() {
                    let animal_pos = res.physics.get_position(*first);
                    (animal_pos - position).normalize_or_zero() * speed
                } else {
                    Vec2::ZERO
                };
                res.physics.set_linear_velocity(self.handle, velocity);
            }
            Status::Dead { ref mut fade_timer } => {
                *fade_timer -= delta;
                if *fade_timer < 0. {
                    res.physics.remove(self.handle);
                    res.physics.remove(self.sensor_handle);
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
                ..
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
