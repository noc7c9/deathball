use macroquad::prelude::*;

use crate::{
    entities::GenerationalIndex, groups, health_bar::HealthBar, physics, spritesheet::Sprite,
    Resources,
};

const FADE_TIME: f32 = 1.;

const PRE_ATTACK_FLASH: f32 = 0.4;
const PRE_ATTACK_DURATION: f32 = PRE_ATTACK_FLASH * 3.;

const ATTACK_DURATION: f32 = 1.;
const ATTACK_OFFSET_START: f32 = 6.;
const ATTACK_OFFSET_END: f32 = 56.;

const HEALTH_BAR_SIZE: (f32, f32) = (32., 14.);
const HEALTH_BAR_OFFSET: (f32, f32) = (16., 36.);

enum Status {
    Alive {
        health_bar: HealthBar,
        health: u16,
        max_health: u16,

        speed: f32,

        attack: Attack,
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
    pub attack_impulse: f32,
}

#[derive(Clone, Copy)]
pub struct Variant {
    _name: &'static str,
    sprite: (f32, f32),
    health: u16,
    speed: f32,
    detection_range: f32,
    attack_impulse: f32,
    attack_cooldown: f32,
}

impl Enemy {
    pub const VARIANTS: [Variant; 6] = [
        Variant {
            _name: "demon",
            sprite: (5., 0.),
            health: 200,
            speed: 75.,
            detection_range: 600.,
            attack_impulse: 500.,
            attack_cooldown: 5.,
        },
        Variant {
            _name: "demon_boss",
            sprite: (5., 0.),
            health: 400,
            speed: 50.,
            detection_range: 6000.,
            attack_impulse: 1000.,
            attack_cooldown: 5.,
        },
        Variant {
            _name: "farmer",
            sprite: (1., 0.),
            health: 10,
            speed: 50.,
            detection_range: 300.,
            attack_impulse: 500.,
            attack_cooldown: 10.,
        },
        Variant {
            _name: "police",
            sprite: (2., 0.),
            health: 20,
            speed: 50.,
            detection_range: 400.,
            attack_impulse: 500.,
            attack_cooldown: 10.,
        },
        Variant {
            _name: "snowman",
            sprite: (4., 0.),
            health: 25,
            speed: 25.,
            detection_range: 600.,
            attack_impulse: 500.,
            attack_cooldown: 7.,
        },
        Variant {
            _name: "soldier",
            sprite: (3., 0.),
            health: 50,
            speed: 60.,
            detection_range: 400.,
            attack_impulse: 700.,
            attack_cooldown: 9.,
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
        let collider = physics::ball(16.)
            .mass(1_000_000_000.)
            .lock_rotations()
            .contact_events();
        let handle = res.physics.add_dynamic(idx, collider, position);

        let collider = physics::ball(variant.detection_range).intersection_events();
        let sensor_handle = res.physics.add_sensor(idx, collider, position);

        Enemy {
            idx,
            sprite,
            handle,
            sensor_handle,
            nearby_animals: Vec::new(),
            attack_impulse: variant.attack_impulse,
            status: Status::Alive {
                health_bar: HealthBar::new(HEALTH_BAR_SIZE.into(), HEALTH_BAR_OFFSET.into()),
                health: variant.health,
                max_health: variant.health,

                speed: variant.speed,

                attack: Attack::new(idx, res, variant.attack_cooldown),
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
                ref mut attack,
                ..
            } => {
                health_bar.update(delta);

                let position = res.physics.get_position(self.handle);

                attack.update(res, delta, position, self.nearby_animals.first());

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

                ref attack,
                ..
            } => {
                self.sprite
                    .draw_tint(position, rotation, attack.enemy_tint());
                health_bar.draw(position, health as f32 / max_health as f32);
                attack.draw(res);
            }
            Status::Dead { fade_timer } => {
                let alpha = fade_timer / FADE_TIME;
                self.sprite.draw_alpha(position, rotation, alpha);
            }
        }
    }
}

struct Attack {
    idx: GenerationalIndex,
    sprite: Sprite,

    cooldown: f32,

    status: AttackStatus,
}

enum AttackStatus {
    Charging {
        timer: f32,
    },
    PreAttack {
        timer: f32,
        direction: Vec2,
        handle: physics::SensorHandle,
    },
    InProgress {
        timer: f32,
        direction: Vec2,
        handle: physics::SensorHandle,
    },
}

impl Attack {
    fn new(enemy_idx: GenerationalIndex, res: &mut Resources, cooldown: f32) -> Self {
        let idx = enemy_idx.with_group(groups::ENEMY_ATTACK);

        let sprite = res.assets.enemies.sprite((7., 5.).into());

        Attack {
            idx,
            sprite,
            cooldown,
            status: AttackStatus::Charging { timer: 0. },
        }
    }

    fn enemy_tint(&self) -> Color {
        if let AttackStatus::PreAttack { timer, .. } = self.status {
            // flash red repeatedly
            let t = (timer % PRE_ATTACK_FLASH) * (1. / PRE_ATTACK_FLASH);
            Color::new(1., t, t, 1.)
        } else {
            WHITE
        }
    }

    fn update(
        &mut self,
        res: &mut Resources,
        delta: f32,
        enemy_position: Vec2,
        target: Option<&physics::Handle>,
    ) {
        let get_direction_to = |target| {
            let target_position = res.physics.get_position(target);
            (target_position - enemy_position).normalize_or_zero()
        };

        match self.status {
            AttackStatus::Charging { ref mut timer } => {
                // charge attack
                *timer += delta;
                if *timer < self.cooldown {
                    return;
                }

                if let Some(&target) = target {
                    let direction = get_direction_to(target);

                    let collider = physics::ball(16.).intersection_events();
                    // spawn off screen so that it won't appear teleport on the first frame
                    let position = Vec2::ONE * 99999999.;
                    let handle = res.physics.add_sensor(self.idx, collider, position);

                    self.status = AttackStatus::PreAttack {
                        timer: 0.,
                        direction,
                        handle,
                    };
                }
            }
            AttackStatus::PreAttack {
                ref mut timer,
                direction,
                handle,
            } => {
                *timer += delta;
                if *timer > PRE_ATTACK_DURATION {
                    self.status = AttackStatus::InProgress {
                        timer: 0.,
                        direction,
                        handle,
                    };
                }
            }
            AttackStatus::InProgress {
                ref mut timer,
                ref mut direction,
                handle,
            } => {
                *timer += delta;
                if *timer > ATTACK_DURATION {
                    res.physics.remove(handle);
                    self.status = AttackStatus::Charging { timer: 0. };
                    return;
                }

                // if we have (still) a target, aim for it's updated position
                if let Some(&target) = target {
                    *direction = get_direction_to(target);
                }

                // move the sensor
                let amount = *timer / ATTACK_DURATION;
                let offset = lerp(ATTACK_OFFSET_START, ATTACK_OFFSET_END, amount) * *direction;
                res.physics.set_position(handle, enemy_position + offset);

                let angle = -direction.angle_between(Vec2::X);
                res.physics.set_rotation(handle, angle);
            }
        }
    }

    fn draw(&self, res: &Resources) {
        if let AttackStatus::InProgress { handle, .. } = self.status {
            let position = res.physics.get_position(handle);
            let rotation = res.physics.get_rotation(handle);
            self.sprite.draw(position, rotation);
        }
    }
}

fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
}
