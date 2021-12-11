use macroquad::prelude::*;

pub struct Input {
    rmb_drag: Option<MouseDrag>,
}

#[derive(Clone, Copy)]
pub struct MouseDrag {
    pub previous: Vec2,
    pub current: Vec2,
}

impl Input {
    pub fn new() -> Self {
        Input { rmb_drag: None }
    }

    pub fn update(&mut self) {
        let is_down = is_mouse_button_down(MouseButton::Right);
        match self.rmb_drag {
            // start drag
            None if is_down => {
                let position = mouse_position().into();
                self.rmb_drag = Some(MouseDrag {
                    previous: position,
                    current: position,
                });
            }
            // continue drag
            Some(ref mut rmb_drag) if is_down => {
                rmb_drag.previous = rmb_drag.current;
                rmb_drag.current = mouse_position().into();
            }
            // end drag
            Some(_) if !is_down => self.rmb_drag = None,
            _ => {}
        }
    }

    pub fn is_spacebar_down(&self) -> bool {
        is_key_down(KeyCode::Space)
    }

    pub fn get_wasd_axes(&self) -> Vec2 {
        let mut delta = vec2(0., 0.);
        if is_key_down(KeyCode::W) {
            delta.y -= 1.0;
        }
        if is_key_down(KeyCode::A) {
            delta.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            delta.y += 1.0;
        }
        if is_key_down(KeyCode::D) {
            delta.x += 1.0;
        }
        delta.normalize_or_zero()
    }

    pub fn is_mouse_middle_button_pressed(&self) -> bool {
        is_mouse_button_down(MouseButton::Middle)
    }

    pub fn get_mouse_right_button_drag(&self) -> Option<MouseDrag> {
        self.rmb_drag
    }

    pub fn get_mouse_left_button_down(&self) -> Option<Vec2> {
        if is_mouse_button_down(MouseButton::Left) {
            Some(mouse_position().into())
        } else {
            None
        }
    }

    pub fn get_mouse_wheel(&self) -> Option<f32> {
        let value = mouse_wheel().1;
        if value == 0.0 {
            None
        } else if cfg!(target_arch = "wasm32") {
            Some(value.clamp(-1., 1.))
        } else {
            Some(value.clamp(-2., 2.))
        }
    }
}
