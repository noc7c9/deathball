use macroquad::prelude::*;

const PAN_SPEED: f32 = 15.;

const INITIAL_ZOOM: f32 = 0.0015;
const ZOOM_FACTOR: f32 = 1.05;
const MIN_ZOOM: f32 = 0.0005;
const MAX_ZOOM: f32 = 0.005;

pub struct Camera {
    pub target: Vec2,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: vec2(0., 0.),
            zoom: INITIAL_ZOOM,
        }
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

    pub fn update(&mut self, input: &crate::input::Input) {
        // Mouse Panning
        if let Some(drag) = input.get_mouse_right_button_drag() {
            let previous = self.screen_to_world(drag.previous);
            let current = self.screen_to_world(drag.current);
            self.target += previous - current;
        }
        // WASD Panning
        else {
            self.target += input.get_wasd_axes() * PAN_SPEED;
        }

        // Mouse Zoom
        if let Some(amount) = input.get_mouse_wheel() {
            self.zoom = (self.zoom * ZOOM_FACTOR.powf(amount)).clamp(MIN_ZOOM, MAX_ZOOM);
        }
    }
}
