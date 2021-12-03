use macroquad::prelude::*;

use crate::{levels, scenes, Resources};

use super::{Scene, SceneChange};

const BACKGROUND_COLOR: Color = Color::new(0.071, 0.219, 0.369, 1.0);
const CHATTER_DOTS: &str = ". . . ";
const CHATTER_MESSAGES: [&str; 11] = [
    "We need to deal with the animal menance",
    "You must construct additional pylons",
    "Why are my horses so angry",
    "Send help",
    "No",
    "Maybe",
    "Yes... No",
    "Insufficient funds",
    "The animals are everywhere",
    "They destroyed my stable",
    "They turned me into a newt",
];
const CHATTER_TIME: f32 = 1.;

pub struct LevelSelect {
    chatter: Vec<&'static str>,
    dots: usize,
    timer: f32,
}

impl LevelSelect {
    pub fn boxed() -> Box<Self> {
        Box::new(LevelSelect {
            chatter: Vec::new(),
            dots: 0,
            timer: CHATTER_TIME,
        })
    }
}

impl Scene for LevelSelect {
    fn update(&mut self, res: &mut Resources) -> SceneChange {
        self.timer -= res.delta;
        if self.timer < 0. {
            self.timer = CHATTER_TIME;
            self.dots = match self.dots {
                0 => {
                    self.chatter.push(&CHATTER_DOTS[..2]);
                    1
                }
                1 | 2 => {
                    let len = self.chatter.len();
                    self.chatter[len - 1] = &CHATTER_DOTS[..(2 + self.dots * 2)];
                    self.dots + 1
                }
                _ => {
                    let msg = &CHATTER_MESSAGES[rand::gen_range(0, CHATTER_MESSAGES.len())];
                    let len = self.chatter.len();
                    self.chatter[len - 1] = msg;
                    0
                }
            };
        }

        SceneChange::None
    }

    fn update_ui(&mut self, res: &mut Resources, ctx: &egui::CtxRef) -> SceneChange {
        use egui::*;

        Window::new("score")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::LEFT_TOP, (8., 8.))
            .show(ctx, |ui| {
                ui.label(format!("Current Score: {}", res.score as f32 / 100.));
            });

        Area::new("header")
            .anchor(egui::Align2::CENTER_TOP, (0., 80.))
            .show(ctx, |ui| {
                Resize::default().fixed_size((640., 0.)).show(ui, |ui| {
                    ui.label("UN's Table");
                });
            });

        Window::new("chatter")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, (0., 110.))
            .vscroll(true)
            .fixed_size((600., 230.))
            .show(ctx, |ui| {
                Resize::default().fixed_size((600., 230.)).show(ui, |ui| {
                    for (i, chatter) in self.chatter.iter().enumerate() {
                        let align = if i % 2 == 0 { Align::Min } else { Align::Max };
                        ui.with_layout(Layout::top_down(align), |ui| {
                            ui.label(chatter);
                        });
                    }
                })
            });

        let mut scene_change = SceneChange::None;

        Window::new("buttons")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, (0., -160.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    ui.spacing_mut().button_padding = vec2(0., 40.);

                    let beat_scenario_1 = res.beaten.contains(&levels::Scenario1);
                    let beat_scenario_2 = res.beaten.contains(&levels::Scenario2);
                    let beat_final = res.beaten.contains(&levels::Final);

                    if !beat_scenario_1 && ui.button("attack on humans").clicked() {
                        scene_change =
                            SceneChange::Change(scenes::Combat::boxed(res, levels::Scenario1));
                    }
                    if !beat_scenario_2 && ui.button("down with the foundations").clicked() {
                        scene_change =
                            SceneChange::Change(scenes::Combat::boxed(res, levels::Scenario2));
                    }
                    if beat_scenario_1 && beat_scenario_2 {
                        if !beat_final && ui.button("final level").clicked() {
                            scene_change =
                                SceneChange::Change(scenes::Combat::boxed(res, levels::Final));
                        }
                        if beat_final {
                            let label = format!(
                                "Thanks for playing! You finished with a score of {}",
                                res.score as f32 / 100.
                            );
                            let label = Label::new(label).wrap(false);
                            ui.add(label);
                        }
                    }
                })
            });

        scene_change
    }

    fn draw(&self, _res: &Resources) {
        clear_background(BACKGROUND_COLOR);
    }
}
