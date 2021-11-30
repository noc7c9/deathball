use macroquad::prelude::*;

use crate::{camera::Camera, groups, physics, spritesheet::Sprite, Resources};

pub struct DeathBall {
    handle: physics::SensorHandle,
    sprite: Sprite,
}

impl DeathBall {
    pub fn new(res: &mut Resources, position: Vec2) -> Self {
        let collider = physics::ball(16.).mass(1.).intersection_events();
        let handle = res
            .physics
            .add_sensor(groups::DEATH_BALL, collider, position);
        DeathBall {
            handle,
            sprite: res.assets.animals.sprite(vec2(7., 5.)),
        }
    }

    pub fn get_position(&self, res: &Resources) -> Vec2 {
        res.physics.get_position(self.handle)
    }

    pub fn update(&mut self, res: &mut Resources, camera: &Camera) {
        if let Some(position) = res.input.get_mouse_left_button_down() {
            let position = camera.screen_to_world(position);
            res.physics.set_position(self.handle, position);
        }
    }

    pub fn draw(&self, res: &Resources) {
        let position = res.physics.get_position(self.handle);
        self.sprite.draw(position, 0.);
    }
}
