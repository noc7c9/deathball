use crate::{levels, scenes, Resources};

use super::Scene;

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
        let mut level_to_load = None;
        egui::Window::new("Pick a level to load")
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, (0., 0.))
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

        level_to_load.map(|level| -> Box<dyn Scene> { Box::new(scenes::Combat::new(res, level)) })
    }

    fn draw(&self, _res: &Resources) {}
}
