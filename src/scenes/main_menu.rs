use macroquad::prelude::*;

use crate::{levels, scenes, Resources};

use super::Scene;

const ENABLE_LEVEL_SELECT: bool = false;
const BACKGROUND_COLOR: Color = Color::new(0.243, 0.133, 0.133, 1.0);

pub struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        MainMenu
    }
}

impl Scene for MainMenu {
    fn update(&mut self, _res: &mut Resources) -> Option<Box<dyn Scene>> {
        None
    }

    fn update_ui(&mut self, res: &mut Resources, ctx: &egui::CtxRef) -> Option<Box<dyn Scene>> {
        use egui::*;

        Window::new("title text")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, (0., 80.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    ui.add(
                        Label::new("GIANT HORSE DEATHBALL")
                            .text_style(TextStyle::Heading)
                            .wrap(false),
                    );

                    let texture = res.assets.icon;
                    texture.set_filter(FilterMode::Nearest);
                    let texture_id = texture.raw_miniquad_texture_handle().gl_internal_id();
                    ui.image(TextureId::User(texture_id as u64), vec2(224., 224.));
                })
            });

        Window::new("buttons")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, (0., -75.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    ui.spacing_mut().button_padding = vec2(0., 80.);

                    ui.button("New Game");

                    ui.button("Quit");
                })
            });

        if ENABLE_LEVEL_SELECT {
            let mut level_to_load = None;
            Window::new("Pick a level to load")
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::LEFT_TOP, (16., 16.))
                .show(ctx, |ui| {
                    if ui.button("Test").clicked() {
                        level_to_load = Some(levels::test::init(res));
                    } else if ui.button("Tutorial Scenario").clicked() {
                        level_to_load = Some(levels::tutorial_scenario::init(res));
                    } else if ui.button("Scenario 1").clicked() {
                        level_to_load = Some(levels::scenario_1::init(res));
                    } else if ui.button("Scenario 2").clicked() {
                        level_to_load = Some(levels::scenario_2::init(res));
                    } else if ui.button("Final Scenario").clicked() {
                        level_to_load = Some(levels::final_scenario::init(res));
                    }
                });
            return level_to_load
                .map(|level| -> Box<dyn Scene> { Box::new(scenes::Combat::new(res, level)) });
        }

        None
    }

    fn draw(&self, _res: &Resources) {
        clear_background(BACKGROUND_COLOR);
    }
}
