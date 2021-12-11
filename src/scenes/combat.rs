use macroquad::prelude::*;

use crate::{
    animals::Animal,
    audio::bgm,
    background::Background,
    buildings::Building,
    camera::Camera,
    death_ball::DeathBall,
    enemies::Enemy,
    entities::Entities,
    groups,
    hit_effect::HitEffect,
    levels::Level,
    objectives::Objective,
    physics::{PhysicsEvent, PhysicsEventKind},
    scenes, Resources,
};

use super::{Scene, SceneChange};

const PAN_SPEED: f32 = 15.;

const INITIAL_ZOOM: f32 = 0.0015;
const ZOOM_FACTOR: f32 = 1.05;
const MIN_ZOOM: f32 = 0.00035;
const MAX_ZOOM: f32 = 0.005;

const LOSE_TIME: f32 = 5.;

#[derive(PartialEq)]
enum Status {
    Playing,
    Losing { timer: f32 },
    HasLost,
    HasWon,
}

pub struct Combat {
    camera: Camera,
    level: Level,
    bgm: bgm::Track,
    objective: Objective,
    background: Background,
    death_ball: DeathBall,
    animals: Entities<Animal, { groups::ANIMAL }>,
    buildings: Entities<Building, { groups::BUILDING }>,
    enemies: Entities<Enemy, { groups::ENEMY }>,
    hit_effects: Entities<HitEffect, { groups::HIT_EFFECT }>,
    death_ball_size: u8,
    score: f32,
    status: Status,
}

impl Combat {
    pub fn boxed(res: &mut Resources, level: Level) -> Box<Self> {
        res.physics.reset();
        let data = level.init(res);
        let death_ball = DeathBall::new(res, Vec2::ZERO);
        let hit_effects = Entities::<HitEffect, { groups::HIT_EFFECT }>::new();
        Box::new(Combat {
            camera: Camera::new(Vec2::ZERO, INITIAL_ZOOM),
            level,
            bgm: data.bgm,
            objective: data.objective,
            background: data.background,
            animals: data.animals,
            buildings: data.buildings,
            enemies: data.enemies,
            death_ball,
            hit_effects,
            death_ball_size: 0,
            score: data.max_score as f32,
            status: Status::Playing,
        })
    }
}

impl Combat {
    fn update_death_ball_size(&mut self) {
        self.death_ball_size = self
            .animals
            .into_iter()
            .filter(|a| a.is_affected_by_death_ball)
            .count() as u8;
    }
}

impl Scene for Combat {
    fn on_enter(&mut self, res: &mut Resources) {
        res.audio.bgm.play(self.bgm);
    }

    fn update(&mut self, res: &mut Resources) -> SceneChange {
        // Update camera
        {
            // Mouse Panning
            if let Some(drag) = res.input.get_mouse_right_button_drag() {
                let previous = self.camera.screen_to_world(drag.previous);
                let current = self.camera.screen_to_world(drag.current);
                self.camera.target += previous - current;
            }
            // WASD Panning
            else {
                self.camera.target += res.input.get_wasd_axes() * PAN_SPEED;
            }

            // Mouse Zoom
            if let Some(amount) = res.input.get_mouse_wheel() {
                self.camera.zoom =
                    (self.camera.zoom * ZOOM_FACTOR.powf(amount)).clamp(MIN_ZOOM, MAX_ZOOM);
            }

            if res.input.is_mouse_middle_button_pressed() {
                self.camera.zoom = INITIAL_ZOOM;
                self.camera.target = self.death_ball.get_position(res);
            }
        }

        // Update entities
        self.death_ball.update(res, &self.camera);
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

        // handle status changes
        match self.status {
            Status::Playing | Status::Losing { .. } if self.objective.is_complete() => {
                self.status = Status::HasWon;
            }
            Status::Playing if self.death_ball_size == 0 => {
                self.status = Status::Losing { timer: LOSE_TIME };
            }
            Status::Losing { .. } if self.death_ball_size > 0 => {
                self.status = Status::Playing;
            }
            Status::Losing { ref mut timer } => {
                *timer -= res.delta;
                if *timer < 0. {
                    self.status = Status::HasLost;
                }
            }
            _ => {}
        }

        if !matches!(self.status, Status::HasWon) {
            self.score = (self.score - res.delta * 100.).max(0.);
        }

        // handle scene changing
        if matches!(self.status, Status::HasLost) && res.input.is_spacebar_down() {
            return SceneChange::Change(scenes::Combat::boxed(res, self.level));
        }
        if matches!(self.status, Status::HasWon) && res.input.is_spacebar_down() {
            res.beaten.insert(self.level);
            res.score += self.score.floor() as u32;
            return SceneChange::Change(scenes::LevelSelect::boxed(res));
        }

        SceneChange::None
    }

    fn handle_physics_event(&mut self, res: &mut Resources, event: PhysicsEvent) {
        let idx1 = res.physics.get_idx(event.collider1);
        let idx2 = res.physics.get_idx(event.collider2);

        // DeathBall with Animal
        if idx1 == groups::DEATH_BALL && idx2.group() == groups::ANIMAL {
            let animal = &mut self.animals[idx2];
            animal.is_affected_by_death_ball = true;

            self.update_death_ball_size();
            self.objective
                .on_update_death_ball_count(self.death_ball_size);
            return;
        }

        // Animal with Building
        if idx1.group() == groups::ANIMAL && idx2.group() == groups::BUILDING {
            let animal = &mut self.animals[idx1];
            let building = &mut self.buildings[idx2];

            if let PhysicsEventKind::ContactStart { point } = event.kind {
                let just_destroyed = building.damage(animal.damage);
                if just_destroyed {
                    self.objective.on_destroy_building();
                    res.audio.play_killed_sfx();
                } else {
                    res.audio.play_hit_sfx();
                }

                // spawn hit effects on contact
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
                    let just_killed = enemy.damage(animal.damage);
                    if just_killed {
                        self.objective.on_kill_enemy();
                        res.audio.play_killed_sfx();
                    } else {
                        res.audio.play_hit_sfx();
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

            self.update_death_ball_size();
            self.objective
                .on_update_death_ball_count(self.death_ball_size);
        }
    }

    fn update_ui(&mut self, _res: &mut Resources, ctx: &egui::CtxRef) -> SceneChange {
        use egui::*;

        Window::new("score")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, (0., 8.))
            .show(ctx, |ui| {
                Resize::default().fixed_size((300., 0.)).show(ui, |ui| {
                    ui.columns(2, |cols| {
                        cols[0].with_layout(Layout::top_down(Align::Center), |ui| {
                            ui.label("Score:");
                        });
                        cols[1].with_layout(Layout::top_down(Align::Center), |ui| {
                            ui.label(format!("{:.2}", self.score as f32 / 100.));
                        });
                    });
                });
            });

        Area::new("objective complete")
            .anchor(egui::Align2::CENTER_TOP, (0., 64.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    if let Status::HasLost = self.status {
                        ui.label("You Lose!");
                        ui.label("Press Spacebar to retry.");
                    } else if let Status::HasWon = self.status {
                        ui.label("You Win!");
                        ui.label("Press Spacebar to go to next screen.");
                    }
                });
            });

        Window::new("objective")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::LEFT_BOTTOM, (8., -8.))
            .show(ctx, |ui| {
                ui.columns(2, |cols| {
                    cols[0].label("Objective:");
                    cols[0].label("Current:");

                    cols[1].add(Label::new(self.objective.to_string()).wrap(false));
                    cols[1].label(self.objective.current());
                });
            });

        Window::new("size")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, (-8., -8.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    if let Status::Losing { timer } = self.status {
                        ui.label("Deathball Is Dissolved");
                        ui.label(timer.ceil());
                    } else {
                        ui.label("Deathball Count");
                        ui.label(self.death_ball_size);
                    }
                });
            });

        SceneChange::None
    }

    fn draw(&self, res: &Resources) {
        self.camera.enable();

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

        if crate::debug::DRAW_COLLIDERS {
            res.physics.draw_colliders();
        }

        self.camera.disable();
    }
}
