use macroquad::prelude::*;

use crate::{
    animals::Animal,
    background::Background,
    buildings::Building,
    death_ball::DeathBall,
    enemies::Enemy,
    entities::Entities,
    groups,
    hit_effect::HitEffect,
    levels::Level,
    objectives::Objective,
    physics::{PhysicsEvent, PhysicsEventKind},
    Resources,
};

use super::Scene;

pub struct Combat {
    objective: Objective,
    background: Background,
    death_ball: DeathBall,
    animals: Entities<Animal, { groups::ANIMAL }>,
    buildings: Entities<Building, { groups::BUILDING }>,
    enemies: Entities<Enemy, { groups::ENEMY }>,
    hit_effects: Entities<HitEffect, { groups::HIT_EFFECT }>,
}

impl Combat {
    pub fn new(res: &mut Resources, level: Level) -> Self {
        let death_ball = DeathBall::new(res, Vec2::ZERO);
        let hit_effects = Entities::<HitEffect, { groups::HIT_EFFECT }>::new();
        Combat {
            objective: level.objective,
            background: level.background,
            animals: level.animals,
            buildings: level.buildings,
            enemies: level.enemies,
            death_ball,
            hit_effects,
        }
    }
}

impl Combat {
    fn get_death_ball_size(&self) -> u8 {
        let size = self
            .animals
            .into_iter()
            .filter(|a| a.is_affected_by_death_ball)
            .count();
        size as u8
    }
}

impl Scene for Combat {
    fn update(&mut self, res: &mut Resources) -> Option<Box<dyn Scene>> {
        // Update entities
        self.death_ball.update(res);
        for animal in &mut self.animals {
            animal.update(res, &self.death_ball);
        }
        for building in &mut self.buildings {
            building.update(res, &mut self.animals);
        }
        for enemy in &mut self.enemies {
            enemy.update(res);
        }
        for hit_effect in &mut self.hit_effects {
            hit_effect.update(res);
        }

        // Clear deleted entities
        for idx in res.deleted.drain(..) {
            match idx.group() {
                groups::ANIMAL => self.animals.remove(idx),
                groups::BUILDING => self.buildings.remove(idx),
                groups::ENEMY => self.enemies.remove(idx),
                groups::HIT_EFFECT => self.hit_effects.remove(idx),
                _ => {}
            };
        }

        None
    }

    fn handle_physics_event(&mut self, res: &mut Resources, event: PhysicsEvent) {
        let idx1 = res.physics.get_idx(event.collider1);
        let idx2 = res.physics.get_idx(event.collider2);

        // DeathBall with Animal
        if idx1 == groups::DEATH_BALL && idx2.group() == groups::ANIMAL {
            let animal = &mut self.animals[idx2];
            animal.is_affected_by_death_ball = true;

            self.objective
                .on_update_death_ball_count(self.get_death_ball_size());

            return;
        }

        // Animal with Building
        if idx1.group() == groups::ANIMAL && idx2.group() == groups::BUILDING {
            let animal = &mut self.animals[idx1];
            let building = &mut self.buildings[idx2];
            building.damage(animal.damage);

            if building.is_destroyed() {
                self.objective.on_destroy_building();
            }

            // spawn hit effects on contact
            if let PhysicsEventKind::ContactStart { point } = event.kind {
                self.hit_effects.push(|idx| HitEffect::new(idx, point));
            }

            return;
        }

        // Animal with Enemy
        if idx1.group() == groups::ANIMAL && idx2.group() == groups::ENEMY {
            let animal = &mut self.animals[idx1];
            let enemy = &mut self.enemies[idx2];

            match event.kind {
                // will only happen for detection range sensor
                PhysicsEventKind::IntersectStart => enemy.add_nearby(event.collider1),
                PhysicsEventKind::IntersectEnd => enemy.remove_nearby(event.collider1),

                // will only happen for collision body
                PhysicsEventKind::ContactStart { point } => {
                    enemy.damage(animal.damage);

                    if enemy.is_dead() {
                        self.objective.on_kill_enemy();
                    }

                    // spawn hit effects on contact
                    self.hit_effects.push(|idx| HitEffect::new(idx, point));
                }
                _ => {}
            }

            return;
        }

        // Animal with Enemy Attacks
        if idx1.group() == groups::ANIMAL && idx2.group() == groups::ENEMY_ATTACK {
            let animal = &mut self.animals[idx1];
            let animal_handle = event.collider1;
            let enemy = &self.enemies[idx2.with_group(groups::ENEMY)];
            let enemy_handle = event.collider2;

            let animal_pos = res.physics.get_position(animal_handle);
            let enemy_pos = res.physics.get_position(enemy_handle);
            let direction = (animal_pos - enemy_pos).normalize_or_zero();

            animal.is_affected_by_death_ball = false;
            res.physics
                .apply_impulse(animal_handle, direction * enemy.attack_impulse);

            self.objective
                .on_update_death_ball_count(self.get_death_ball_size());
        }
    }

    fn update_ui(&mut self, _res: &mut Resources, ctx: &egui::CtxRef) -> Option<Box<dyn Scene>> {
        egui::Window::new("Objective").show(ctx, |ui| {
            ui.label(self.objective.progress_string());
        });

        None
    }

    fn draw(&self, res: &Resources) {
        self.background.draw();
        self.death_ball.draw(res);
        for hit_effect in &self.hit_effects {
            hit_effect.draw();
        }
        for animal in &self.animals {
            animal.draw(res);
        }
        for enemy in &self.enemies {
            enemy.draw(res);
        }
        for building in &self.buildings {
            building.draw(res);
        }
    }
}
