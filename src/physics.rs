//! The goal of this module is to wrap rapier2d so that
//! - use glam vectors so it works nicer with macroquad
//! - exposes the minimal amount of complexity necessary for

use std::sync::Mutex;

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::entities::GenerationalIndex;

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

    events: Vec<IntersectionEvent>,
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
            events: Vec::new(),
        }
    }

    pub fn update(&mut self, events: &mut Vec<PhysicsEvent>) {
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
            &RawEventCollector(Mutex::new(&mut self.events)),
        );

        for event in self.events.drain(..) {
            events.push(PhysicsEvent::from_intersection_event(
                &self.collider_set,
                event,
            ))
        }
    }

    pub fn add_static(
        &mut self,
        idx: GenerationalIndex,
        collider: MyColliderBuilder,
        position: Vec2,
    ) -> StaticHandle {
        let mut collider = collider.0.user_data(idx.to_u128()).build();
        collider.set_translation(position.into());
        let collider_handle = self.collider_set.insert(collider);
        StaticHandle(collider_handle)
    }

    pub fn add_sensor(
        &mut self,
        idx: GenerationalIndex,
        collider: MyColliderBuilder,
        position: Vec2,
    ) -> SensorHandle {
        let mut collider = collider.0.sensor(true).user_data(idx.to_u128()).build();
        collider.set_translation(position.into());
        let collider_handle = self.collider_set.insert(collider);
        SensorHandle(collider_handle)
    }

    pub fn add_dynamic(
        &mut self,
        idx: GenerationalIndex,
        collider: MyColliderBuilder,
        position: Vec2,
    ) -> DynamicHandle {
        let collider = collider.0.user_data(idx.to_u128()).build();
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

    pub fn get_idx(&self, handle: impl Into<Handle>) -> GenerationalIndex {
        let collider = &self.collider_set[handle.into().collision_handle()];
        GenerationalIndex::from_u128(collider.user_data)
    }

    pub fn set_position(&mut self, handle: impl Into<Handle>, position: Vec2) {
        match handle.into() {
            Handle::Static(StaticHandle(handle)) | Handle::Sensor(SensorHandle(handle)) => {
                let body = &mut self.collider_set[handle];
                body.set_translation(position.into());
            }
            Handle::Dynamic(handle) => {
                let body = &mut self.rigid_body_set[handle.1];
                body.set_translation(position.into(), true);
            }
        }
    }

    pub fn get_position(&self, handle: impl Into<Handle>) -> Vec2 {
        match handle.into() {
            Handle::Static(StaticHandle(handle)) | Handle::Sensor(SensorHandle(handle)) => {
                let body = &self.collider_set[handle];
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
            Handle::Static(StaticHandle(handle)) | Handle::Sensor(SensorHandle(handle)) => {
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

    pub fn apply_impulse(&mut self, handle: DynamicHandle, impulse: Vec2) {
        self.rigid_body_set[handle.1].apply_impulse(impulse.into(), true);
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

pub struct PhysicsEvent {
    pub collider1: Handle,
    pub collider2: Handle,
}

impl PhysicsEvent {
    fn from_intersection_event(collider_set: &ColliderSet, event: IntersectionEvent) -> Self {
        let to_handle = |collider_handle: ColliderHandle| {
            let collider = &collider_set[collider_handle];
            if let Some(rigid_body_handle) = collider.parent() {
                Handle::Dynamic(DynamicHandle(collider_handle, rigid_body_handle))
            } else if collider.is_sensor() {
                Handle::Sensor(SensorHandle(collider_handle))
            } else {
                Handle::Static(StaticHandle(collider_handle))
            }
        };

        PhysicsEvent {
            collider1: to_handle(event.collider1),
            collider2: to_handle(event.collider2),
        }
    }
}

// Despite being single-threaded Rapier2d requires Sync
// (see: https://github.com/dimforge/rapier/issues/253)
struct RawEventCollector<'a>(Mutex<&'a mut Vec<IntersectionEvent>>);

impl<'a> EventHandler for RawEventCollector<'a> {
    fn handle_intersection_event(&self, event: IntersectionEvent) {
        if event.intersecting {
            self.0.lock().unwrap().push(event);
        }
    }

    fn handle_contact_event(&self, _event: ContactEvent, _pair: &ContactPair) {
        todo!()
    }
}

pub struct MyColliderBuilder(ColliderBuilder);

impl MyColliderBuilder {
    pub fn events(self, intersection_events: bool, contact_events: bool) -> Self {
        let mut flags = ActiveEvents::empty();
        if intersection_events {
            flags |= ActiveEvents::INTERSECTION_EVENTS;
        }
        if contact_events {
            flags |= ActiveEvents::CONTACT_EVENTS;
        }
        MyColliderBuilder(self.0.active_events(flags))
    }

    pub fn mass(mut self, mass: f32) -> Self {
        // ensure the builder has a mass_properties
        if self.0.mass_properties.is_none() {
            self.0.mass_properties = Some(self.0.shape.mass_properties(1.));
        }

        (*self.0.mass_properties.as_mut().unwrap()).set_mass(mass, true);

        self
    }
}

pub fn cuboid(size: Vec2) -> MyColliderBuilder {
    MyColliderBuilder(ColliderBuilder::cuboid(size.x / 2., size.y / 2.))
}

pub fn ball(radius: f32) -> MyColliderBuilder {
    MyColliderBuilder(ColliderBuilder::ball(radius))
}

#[derive(Clone, Copy)]
pub struct StaticHandle(ColliderHandle);

#[derive(Clone, Copy)]
pub struct SensorHandle(ColliderHandle);

#[derive(Clone, Copy)]
pub struct DynamicHandle(ColliderHandle, RigidBodyHandle);

#[derive(Clone, Copy)]
pub enum Handle {
    Static(StaticHandle),
    Sensor(SensorHandle),
    Dynamic(DynamicHandle),
}

impl Handle {
    fn collision_handle(self) -> ColliderHandle {
        match self {
            Handle::Static(StaticHandle(handle)) => handle,
            Handle::Sensor(SensorHandle(handle)) => handle,
            Handle::Dynamic(DynamicHandle(handle, ..)) => handle,
        }
    }
}

impl From<StaticHandle> for Handle {
    fn from(handle: StaticHandle) -> Handle {
        Handle::Static(handle)
    }
}

impl From<SensorHandle> for Handle {
    fn from(handle: SensorHandle) -> Handle {
        Handle::Sensor(handle)
    }
}

impl From<DynamicHandle> for Handle {
    fn from(handle: DynamicHandle) -> Handle {
        Handle::Dynamic(handle)
    }
}
