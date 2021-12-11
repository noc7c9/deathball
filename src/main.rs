use macroquad::prelude::*;

mod assets;
mod audio;
mod camera;
mod entities;
mod input;
mod levels;
mod physics;
mod scenes;
mod spritesheet;

use assets::Assets;
use audio::AudioManager;
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

mod debug {
    pub const AUTO_COMPLETE_OBJECTIVES: bool = false;
    pub const DISABLE_BGM: bool = false;
    pub const DISABLE_SFX: bool = false;
    pub const DRAW_COLLIDERS: bool = false;
    pub const ENABLE_LEVEL_SELECT: bool = false;
    pub const SHOW_FPS: bool = false;
}

const FPS_SMOOTHING: f32 = 0.9;

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
    audio: AudioManager,
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
    let mut assets = loading_screen().await;
    let mut res = Resources {
        audio: AudioManager::new(&mut assets),
        assets,
        input: Input::new(),
        physics: Physics::new(),
        deleted: Vec::new(),
        delta: 0.,
        score: 0,
        beaten: Default::default(),
    };
    let mut physics_events: Vec<PhysicsEvent> = Vec::new();

    let mut fps = 0.;

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

    scene.on_enter(&mut res);

    loop {
        res.delta = get_frame_time();

        new_scene = scene.update(&mut res);

        // Update subsystems
        res.input.update();
        res.physics.update(res.delta, &mut physics_events);

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
                scene.on_enter(&mut res);
            }
        }

        if crate::debug::SHOW_FPS {
            fps = (fps * FPS_SMOOTHING) + ((1. / res.delta) * (1. - FPS_SMOOTHING));
            let text = format!("FPS: {:>6.2}", fps);
            draw_text(&text, screen_width() - 86., 16., 16., WHITE);
        }

        next_frame().await
    }
}

async fn loading_screen() -> Assets {
    egui_macroquad::cfg(|ctx| {
        use egui::*;

        let mut fonts = FontDefinitions::default();
        fonts
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 40.));
        ctx.set_fonts(fonts);
    });

    let mut loader = Assets::loader();

    loop {
        match loader.progress().await {
            assets::Progress::InProgress(percent) => {
                egui_macroquad::ui(|ctx| {
                    use egui::*;
                    Area::new("loading")
                        .anchor(Align2::CENTER_CENTER, (0., 0.))
                        .show(ctx, |ui| {
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::TopDown),
                                |ui| {
                                    ui.add(
                                        Label::new(format!("Loading: {:.1}%", percent * 100.))
                                            .text_style(TextStyle::Heading),
                                    );
                                },
                            );
                        });
                });

                egui_macroquad::draw();
            }
            assets::Progress::Complete(assets) => return assets,
        }

        next_frame().await;
    }
}
