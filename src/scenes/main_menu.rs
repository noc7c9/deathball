use macroquad::prelude::*;

use crate::{audio::bgm, levels, scenes, Resources};

use super::{Scene, SceneChange};

const BACKGROUND_COLOR: Color = Color::new(0.243, 0.133, 0.133, 1.0);

pub struct MainMenu;

impl MainMenu {
    pub fn boxed() -> Box<Self> {
        Box::new(MainMenu)
    }
}

impl Scene for MainMenu {
    fn on_enter(&mut self, res: &mut Resources) {
        res.audio.bgm.play(bgm::GiantHorseDeathball);
    }

    fn update(&mut self, _res: &mut Resources) -> SceneChange {
        SceneChange::None
    }

    fn update_ui(&mut self, res: &mut Resources, ctx: &egui::CtxRef) -> SceneChange {
        use egui::*;

        let mut scene_change = SceneChange::None;

        Area::new("Main Menu")
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, (0., 0.))
            .show(ctx, |ui| {
                Resize::default()
                    .auto_sized()
                    .max_size((640., 420.))
                    .show(ui, |ui| {
                        Frame::window(&ctx.style()).show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                                let text = RichText::new("GIANT HORSE DEATHBALL").heading();
                                ui.add(Label::new(text).wrap(false));

                                ui.allocate_exact_size(vec2(0., 16.), Sense::hover());

                                let texture = res.assets.icon;
                                texture.set_filter(FilterMode::Nearest);
                                let texture_id =
                                    texture.raw_miniquad_texture_handle().gl_internal_id();
                                ui.image(TextureId::User(texture_id as u64), vec2(224., 224.));
                            })
                        });

                        ui.allocate_exact_size(vec2(0., 64.), Sense::hover());

                        Frame::window(&ctx.style()).show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                                ui.spacing_mut().button_padding = vec2(0., 32.);

                                if ui.button("New Game").clicked() {
                                    scene_change = SceneChange::Change(scenes::Combat::boxed(
                                        res,
                                        levels::Tutorial,
                                    ));
                                }
                                if !cfg!(target_arch = "wasm32") && ui.button("Quit").clicked() {
                                    scene_change = SceneChange::Quit;
                                }
                            })
                        });
                    });
            });

        if crate::debug::ENABLE_LEVEL_SELECT {
            let mut level_to_load = None;
            Window::new("Load Level")
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::LEFT_TOP, (16., 16.))
                .show(ctx, |ui| {
                    if ui.button("Test").clicked() {
                        level_to_load = Some(levels::Test);
                    } else if ui.button("Tutorial Scenario").clicked() {
                        level_to_load = Some(levels::Tutorial);
                    } else if ui.button("Scenario 1").clicked() {
                        level_to_load = Some(levels::Scenario1);
                    } else if ui.button("Scenario 2").clicked() {
                        level_to_load = Some(levels::Scenario2);
                    } else if ui.button("Final Scenario").clicked() {
                        level_to_load = Some(levels::Final);
                    }
                });
            scene_change = level_to_load.map_or(SceneChange::None, |level| {
                SceneChange::Change(scenes::Combat::boxed(res, level))
            });
        }

        scene_change
    }

    fn draw(&self, _res: &Resources) {
        clear_background(BACKGROUND_COLOR);
    }
}
