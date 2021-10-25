use macroquad::prelude::*;
use rapier2d::prelude::*;

struct Context {
    entities: Vec<Entity>,

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

enum Entity {
    StaticRect {
        width: f32,
        height: f32,
        handle: ColliderHandle,
    },
    RigidCircle {
        radius: f32,
        handle: RigidBodyHandle,
    },
}

impl Entity {
    fn static_rect(ctx: &mut Context, x: f32, y: f32, width: f32, height: f32) -> usize {
        let collider = ColliderBuilder::cuboid(width / 2., height / 2.)
            .translation(vector![x, y])
            .restitution(1.)
            .friction(0.)
            .build();
        let handle = ctx.collider_set.insert(collider);

        let entity = Entity::StaticRect {
            width,
            height,
            handle,
        };

        let idx = ctx.entities.len();
        ctx.entities.push(entity);
        idx
    }

    fn rigid_circle(ctx: &mut Context, x: f32, y: f32, radius: f32) -> usize {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![x, y])
            .ccd_enabled(true)
            .build();
        let handle = ctx.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(radius)
            .restitution(1.)
            .friction(0.)
            .build();
        ctx.collider_set
            .insert_with_parent(collider, handle, &mut ctx.rigid_body_set);

        let entity = Entity::RigidCircle { radius, handle };

        let idx = ctx.entities.len();
        ctx.entities.push(entity);
        idx
    }

    fn draw(&self, ctx: &Context) {
        use Entity::*;
        match self {
            StaticRect {
                width,
                height,
                handle,
            } => {
                let body = &ctx.collider_set[*handle];
                let pos = body.translation();
                draw_rectangle(
                    pos.x - *width / 2.,
                    pos.y - *height / 2.,
                    *width,
                    *height,
                    BLUE,
                );
            }

            RigidCircle { radius, handle } => {
                let body = &ctx.rigid_body_set[*handle];
                let pos = body.translation();
                draw_circle(pos.x, pos.y, *radius, YELLOW);
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

    // Create the boundaries
    let w = screen_width();
    let h = screen_height();
    let cx = w / 2.;
    let cy = h / 2.;
    Entity::static_rect(&mut ctx, cx, 0., w, 10.);
    Entity::static_rect(&mut ctx, cx, h, w, 10.);
    Entity::static_rect(&mut ctx, 0., cy, 10., h);
    Entity::static_rect(&mut ctx, w, cy, 10., h);

    // Create ball
    for _ in 0..100 {
        let x = rand::gen_range(30., screen_width() - 30.);
        let y = rand::gen_range(30., screen_height() - 30.);
        let idx = Entity::rigid_circle(&mut ctx, x, y, 5.);

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

        clear_background(BLACK);

        for entity in &ctx.entities {
            entity.draw(&ctx);
        }

        next_frame().await
    }
}
