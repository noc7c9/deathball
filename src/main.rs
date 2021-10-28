use macroquad::prelude::*;
use rapier2d::prelude::*;

const CAMERA_PAN_SPEED: f32 = 5.;
const CAMERA_DEFAULT_ZOOM: f32 = 0.004;

struct Context {
    entities: Vec<Entity>,

    input: Input,

    camera: Camera,

    physics_pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
}

struct Input;

impl Input {
    fn get_wasd_axes(&self) -> Vec2 {
        let mut delta = vec2(0., 0.);
        if is_key_down(KeyCode::W) {
            delta.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            delta.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            delta.y -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            delta.x += 1.0;
        }
        delta.try_normalize().unwrap_or(Vec2::ZERO)
    }

    fn get_mouse_wheel(&self) -> f32 {
        mouse_wheel().1
    }
}

struct Camera {
    target: Vec2,
    zoom: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            target: vec2(0., 0.),
            zoom: CAMERA_DEFAULT_ZOOM,
        }
    }

    fn enable(&self) {
        set_camera(&Camera2D {
            target: self.target,
            zoom: vec2(self.zoom, self.zoom * screen_width() / screen_height()),
            ..Default::default()
        });
    }

    fn disable(&self) {
        set_default_camera();
    }
}

enum Entity {
    StaticRect {
        size: Vec2,
        color: Color,
        handle: ColliderHandle,
    },
    RigidCircle {
        radius: f32,
        color: Color,
        handle: RigidBodyHandle,
    },
}

impl Entity {
    fn static_rect(ctx: &mut Context, position: Vec2, size: Vec2, color: Color) -> usize {
        let collider = ColliderBuilder::cuboid(size.x / 2., size.y / 2.)
            .translation(position.into())
            .restitution(1.)
            .friction(0.)
            .build();
        let handle = ctx.collider_set.insert(collider);

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
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(position.into())
            .ccd_enabled(true)
            .build();
        let handle = ctx.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(radius)
            .restitution(1.)
            .friction(0.)
            .build();
        ctx.collider_set
            .insert_with_parent(collider, handle, &mut ctx.rigid_body_set);

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
                let body = &ctx.collider_set[handle];
                let pos = Vec2::from(*body.translation()) - size / 2.;
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            }

            RigidCircle {
                radius,
                color,
                handle,
            } => {
                let body = &ctx.rigid_body_set[handle];
                let pos = *body.translation();
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

        input: Input,

        camera: Camera::new(),

        physics_pipeline: PhysicsPipeline::new(),
        gravity: vector![0., 0.],
        integration_parameters: IntegrationParameters::default(),
        island_manager: IslandManager::new(),
        broad_phase: BroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        rigid_body_set: RigidBodySet::new(),
        collider_set: ColliderSet::new(),
        joint_set: JointSet::new(),
        ccd_solver: CCDSolver::new(),
    };

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
            ctx.rigid_body_set[handle].set_linvel(vector![dx, dy], true);
        }
    }

    loop {
        ctx.physics_pipeline.step(
            &ctx.gravity,
            &ctx.integration_parameters,
            &mut ctx.island_manager,
            &mut ctx.broad_phase,
            &mut ctx.narrow_phase,
            &mut ctx.rigid_body_set,
            &mut ctx.collider_set,
            &mut ctx.joint_set,
            &mut ctx.ccd_solver,
            &(),
            &(),
        );

        // read input
        ctx.camera.target += ctx.input.get_wasd_axes() * CAMERA_PAN_SPEED;
        ctx.camera.zoom *= 1.1f32.powf(ctx.input.get_mouse_wheel());

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
