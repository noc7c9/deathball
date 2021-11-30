use macroquad::prelude::*;

pub struct Camera {
    pub target: Vec2,
    pub zoom: f32,
}

impl Camera {
    pub fn new(target: Vec2, zoom: f32) -> Self {
        Self { target, zoom }
    }

    fn get_macroquad_camera(&self) -> Camera2D {
        Camera2D {
            target: self.target,
            zoom: vec2(self.zoom, -self.zoom * screen_width() / screen_height()),
            ..Default::default()
        }
    }

    // fn world_to_screen(&self, point: Vec2) -> Vec2 {
    //     self.get_macroquad_camera().world_to_screen(point)
    // }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.get_macroquad_camera().screen_to_world(point)
    }

    pub fn enable(&self) {
        set_camera(&self.get_macroquad_camera());
    }

    pub fn disable(&self) {
        set_default_camera();
    }
}
