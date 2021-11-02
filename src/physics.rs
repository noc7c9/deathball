//! The goal of this module is to wrap rapier2d so that
//! - use glam vectors so it works nicer with macroquad
//! - exposes the minimal amount of complexity necessary for

use macroquad::prelude::*;
use rapier2d::prelude::*;

pub struct Physics {
    physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            physics_pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn update(&mut self) {
        self.physics_pipeline.step(
            &vector![0., 0.],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        )
    }

    pub fn add_static(&mut self, collider: Collider, position: Vec2) -> StaticHandle {
        let mut collider = collider;
        collider.set_translation(position.into());
        let collider_handle = self.collider_set.insert(collider);
        StaticHandle(collider_handle)
    }

    pub fn add_dynamic(&mut self, collider: Collider, position: Vec2) -> DynamicHandle {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(position.into())
            .ccd_enabled(true)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );
        DynamicHandle(collider_handle, rigid_body_handle)
    }

    pub fn get_position(&self, handle: impl Into<Handle>) -> Vec2 {
        match handle.into() {
            Handle::Static(handle) => {
                let body = &self.collider_set[handle.0];
                (*body.translation()).into()
            }
            Handle::Dynamic(handle) => {
                let body = &self.rigid_body_set[handle.1];
                (*body.translation()).into()
            }
        }
    }

    pub fn get_rotation(&self, handle: impl Into<Handle>) -> f32 {
        use nalgebra::ComplexField;
        match handle.into() {
            Handle::Static(handle) => {
                let body = &self.collider_set[handle.0];
                body.rotation().to_polar().1
            }
            Handle::Dynamic(handle) => {
                let body = &self.rigid_body_set[handle.1];
                body.rotation().to_polar().1
            }
        }
    }

    pub fn set_linear_velocity(&mut self, handle: DynamicHandle, linvel: Vec2) {
        self.rigid_body_set[handle.1].set_linvel(linvel.into(), true);
    }

    pub fn set_angular_velocity(&mut self, handle: DynamicHandle, angvel: f32) {
        self.rigid_body_set[handle.1].set_angvel(angvel, true);
    }

    pub fn draw_colliders(&self) {
        use nalgebra::ComplexField;

        const COLOR: Color = Color::new(0.0, 0.47, 0.95, 0.5);

        for (_, collider) in self.collider_set.iter() {
            let translation = collider.translation();

            match collider.shape().as_typed_shape() {
                TypedShape::Ball(ball) => {
                    draw_circle(translation.x, translation.y, ball.radius, COLOR);
                }
                TypedShape::Cuboid(cuboid) => {
                    if collider.rotation().to_polar().1 != 0. {
                        panic!("drawing rotated rectangles is unsupported");
                    }
                    let size = cuboid.half_extents * 2.;
                    let translation = translation - cuboid.half_extents;
                    draw_rectangle(translation.x, translation.y, size.x, size.y, COLOR);
                }
                _ => panic!("drawing shape is unsupported"),
            }
        }
    }
}

pub fn cuboid(size: Vec2) -> Collider {
    ColliderBuilder::cuboid(size.x / 2., size.y / 2.)
        .restitution(1.)
        .friction(0.)
        .build()
}

pub fn ball(radius: f32) -> Collider {
    ColliderBuilder::ball(radius)
        .restitution(1.)
        .friction(0.)
        .build()
}

#[derive(Clone, Copy)]
pub struct StaticHandle(ColliderHandle);

#[derive(Clone, Copy)]
pub struct DynamicHandle(ColliderHandle, RigidBodyHandle);

#[derive(Clone, Copy)]
pub enum Handle {
    Static(StaticHandle),
    Dynamic(DynamicHandle),
}

impl From<StaticHandle> for Handle {
    fn from(handle: StaticHandle) -> Handle {
        Handle::Static(handle)
    }
}

impl From<DynamicHandle> for Handle {
    fn from(handle: DynamicHandle) -> Handle {
        Handle::Dynamic(handle)
    }
}
