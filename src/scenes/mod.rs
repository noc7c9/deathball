use crate::{physics::PhysicsEvent, Resources};

pub trait Scene {
    fn on_enter(&mut self, _res: &mut Resources) {}

    fn update(&mut self, res: &mut Resources) -> SceneChange;

    fn handle_physics_event(&mut self, _res: &mut Resources, _event: PhysicsEvent) {}

    fn update_ui(&mut self, res: &mut Resources, ctx: &egui::CtxRef) -> SceneChange;

    fn draw(&self, res: &Resources);
}

pub enum SceneChange {
    None,
    Change(Box<dyn Scene>),
    Quit,
}

// individual scenes

mod main_menu;
pub use main_menu::MainMenu;

pub mod combat;
pub use combat::Combat;

pub mod level_select;
pub use level_select::LevelSelect;
