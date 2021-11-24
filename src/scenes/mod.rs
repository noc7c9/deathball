use crate::{physics::PhysicsEvent, Resources};

pub trait Scene {
    fn update(&mut self, res: &mut Resources) -> Option<Box<dyn Scene>>;

    fn handle_physics_event(&mut self, _res: &mut Resources, _event: PhysicsEvent) {}

    fn update_ui(&mut self, res: &mut Resources, ctx: &egui::CtxRef) -> Option<Box<dyn Scene>>;

    fn draw(&self, res: &Resources);
}

// individual scenes

mod main_menu;
pub use main_menu::MainMenu;

pub mod combat;
pub use combat::Combat;
