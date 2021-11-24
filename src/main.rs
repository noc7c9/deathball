use macroquad::prelude::*;

mod assets;
mod camera;
mod entities;
mod input;
mod levels;
mod physics;
mod scenes;
mod spritesheet;

use assets::Assets;
use camera::Camera;
use entities::GenerationalIndex;
use input::Input;
use physics::{Physics, PhysicsEvent};
use scenes::Scene;

mod animals;
mod background;
mod buildings;
mod death_ball;
mod enemies;
mod health;
mod hit_effect;

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
        high_dpi: true,
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

    let mut scene: Box<dyn Scene> = Box::new(scenes::MainMenu::new());
    let mut new_scene: Option<Box<dyn Scene>>;

    egui_macroquad::cfg(|ctx| {
        ctx.set_fonts({
            let mut fonts = egui::FontDefinitions::default();

            fonts
                .font_data
                .insert("font".to_owned(), res.assets.font.take().unwrap().into());

            fonts
                .fonts_for_family
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "font".to_owned());

            fonts
        });
    });

    loop {
        res.delta = get_frame_time();

        new_scene = scene.update(&mut res);

        // Update subsystems
        res.input.update();
        res.camera.update(&res.input);
        res.physics.update(&mut physics_events);

        for event in physics_events.drain(..) {
            scene.handle_physics_event(&mut res, event);
        }

        egui_macroquad::ui(|egui_ctx| {
            new_scene = scene.update_ui(&mut res, egui_ctx);
        });

        // Draw
        res.camera.enable();

        scene.draw(&res);

        if DRAW_COLLIDERS {
            res.physics.draw_colliders();
        }

        egui_macroquad::draw();

        res.camera.disable();

        if let Some(new_scene) = new_scene.take() {
            scene = new_scene;
        }

        next_frame().await
    }
}
