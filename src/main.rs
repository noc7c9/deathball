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
use entities::GenerationalIndex;
use input::Input;
use levels::Level;
use physics::{Physics, PhysicsEvent};
use scenes::{Scene, SceneChange};

mod animals;
mod background;
mod buildings;
mod death_ball;
mod enemies;
mod health;
mod hit_effect;
mod objectives;

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
    physics: Physics,
    deleted: Vec<GenerationalIndex>,
    delta: f32,

    score: u32,
    beaten: std::collections::HashSet<Level>,
}

pub fn window_config() -> Conf {
    Conf {
        window_title: "Giant Horse Deathball".to_owned(),
        window_width: 1600,
        window_height: 900,
        // high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut res = Resources {
        assets: Assets::load().await,
        input: Input::new(),
        physics: Physics::new(),
        deleted: Vec::new(),
        delta: 0.,
        score: 0,
        beaten: Default::default(),
    };
    let mut physics_events: Vec<PhysicsEvent> = Vec::new();

    let mut scene: Box<dyn Scene> = scenes::MainMenu::boxed();
    let mut new_scene;

    egui_macroquad::cfg(|ctx| {
        use egui::*;

        ctx.set_fonts({
            let mut fonts = FontDefinitions::default();

            fonts
                .font_data
                .insert("font".to_owned(), res.assets.font.take().unwrap().into());

            fonts
                .fonts_for_family
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "font".to_owned());

            fonts
                .family_and_size
                .insert(TextStyle::Heading, (FontFamily::Proportional, 40.));

            fonts
                .family_and_size
                .insert(TextStyle::Body, (FontFamily::Proportional, 18.));

            fonts
        });

        let mut style: egui::Style = (*ctx.style()).clone();
        style.spacing.window_padding = vec2(14., 14.);
        style.visuals.window_shadow.extrusion = 0.;
        style.visuals.widgets.noninteractive.bg_stroke.width = 0.;
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_black_alpha(187);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        ctx.set_style(style);
    });

    loop {
        res.delta = get_frame_time();

        new_scene = scene.update(&mut res);

        // Update subsystems
        res.input.update();
        res.physics.update(&mut physics_events);

        for event in physics_events.drain(..) {
            scene.handle_physics_event(&mut res, event);
        }

        egui_macroquad::ui(|egui_ctx| {
            let change = scene.update_ui(&mut res, egui_ctx);
            if !matches!(change, SceneChange::None) {
                new_scene = change
            }
        });

        // Draw
        scene.draw(&res);

        egui_macroquad::draw();

        match new_scene {
            SceneChange::None => {}
            SceneChange::Quit => break,
            SceneChange::Change(new_scene) => {
                scene = new_scene;
            }
        }

        next_frame().await
    }
}
