use macroquad::prelude::*;

mod physics;
use physics::Physics;
mod input;
use input::Input;

const CAMERA_PAN_SPEED: f32 = 15.;
const CAMERA_DEFAULT_ZOOM: f32 = 0.001;
const CAMERA_MIN_ZOOM: f32 = 0.0005;
const CAMERA_MAX_ZOOM: f32 = 0.005;

struct Context {
    entities: Vec<Entity>,
    input: Input,
    camera: Camera,
    physics: Physics,
}

struct Camera {
    target: Vec2,
    zoom: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            target: vec2(0., 0.),
            zoom: 1.,
        }
    }

    fn get_macroquad_camera(&self) -> Camera2D {
        Camera2D {
            target: self.target,
            zoom: vec2(self.zoom, self.zoom * screen_width() / screen_height()),
            ..Default::default()
        }
    }

    // fn world_to_screen(&self, point: Vec2) -> Vec2 {
    //     self.get_macroquad_camera().world_to_screen(point)
    // }

    fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.get_macroquad_camera().screen_to_world(point)
    }

    fn enable(&self) {
        set_camera(&self.get_macroquad_camera());
    }

    fn disable(&self) {
        set_default_camera();
    }
}

enum Entity {
    StaticRect {
        size: Vec2,
        color: Color,
        handle: physics::StaticHandle,
    },
    RigidCircle {
        radius: f32,
        color: Color,
        handle: physics::DynamicHandle,
    },
}

impl Entity {
    fn static_rect(ctx: &mut Context, position: Vec2, size: Vec2, color: Color) -> usize {
        let collider = physics::cuboid(size);
        let handle = ctx.physics.add_static(collider, position);

        let entity = Entity::StaticRect {
            size,
            color,
            handle,
        };

        let idx = ctx.entities.len();
        ctx.entities.push(entity);
        idx
    }

    fn rigid_circle(ctx: &mut Context, position: Vec2, radius: f32, color: Color) -> usize {
        let collider = physics::ball(radius);
        let handle = ctx.physics.add_dynamic(collider, position);

        let entity = Entity::RigidCircle {
            radius,
            color,
            handle,
        };

        let idx = ctx.entities.len();
        ctx.entities.push(entity);
        idx
    }

    fn draw(&self, ctx: &Context) {
        use Entity::*;
        match *self {
            StaticRect {
                size,
                color,
                handle,
            } => {
                let pos = ctx.physics.get_position(handle) - size / 2.;
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            }

            RigidCircle {
                radius,
                color,
                handle,
            } => {
                let pos = ctx.physics.get_position(handle);
                draw_circle(pos.x, pos.y, radius, color);
            }
        }
    }
}

pub fn window_config() -> Conf {
    Conf {
        window_title: "Giant Horse Deathball".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut ctx = Context {
        entities: vec![],
        input: Input::new(),
        camera: Camera::new(),
        physics: Physics::new(),
    };

    ctx.camera.zoom = CAMERA_DEFAULT_ZOOM;

    let screen_size = vec2(screen_width(), screen_height());
    let screen_center = screen_size / 2.;

    // Create the boundaries
    {
        let s = screen_size;
        let c = screen_center;
        Entity::static_rect(&mut ctx, vec2(0., -c.y), vec2(s.x + 10., 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(0., c.y), vec2(s.x + 10., 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(-c.x, 0.), vec2(10., s.y + 10.), BLUE);
        Entity::static_rect(&mut ctx, vec2(c.x, 0.), vec2(10., s.y + 10.), BLUE);
    }

    // Create ball
    for _ in 0..100 {
        let x = rand::gen_range(-screen_center.x + 25., screen_center.x - 25.);
        let y = rand::gen_range(-screen_center.y + 25., screen_center.y - 25.);
        let idx = Entity::rigid_circle(&mut ctx, vec2(x, y), 5., YELLOW);

        let dx = rand::gen_range(-500., 500.);
        let dy = rand::gen_range(-500., 500.);
        if let Entity::RigidCircle { handle, .. } = ctx.entities[idx] {
            ctx.physics.set_linear_velocity(handle, vec2(dx, dy));
        }
    }

    loop {
        ctx.input.update();

        ctx.physics.update();

        // read input
        if let Some(drag) = ctx.input.get_mouse_drag() {
            let previous = ctx.camera.screen_to_world(drag.previous);
            let current = ctx.camera.screen_to_world(drag.current);
            ctx.camera.target += previous - current;
        } else {
            ctx.camera.target += ctx.input.get_wasd_axes() * CAMERA_PAN_SPEED;
        }
        if let Some(amount) = ctx.input.get_mouse_wheel() {
            ctx.camera.zoom =
                (ctx.camera.zoom * 1.1f32.powf(amount)).clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
        }

        ctx.camera.enable();

        // draw
        clear_background(BLACK);
        for entity in &ctx.entities {
            entity.draw(&ctx);
        }

        ctx.camera.disable();

        next_frame().await
    }
}
