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

    events: Vec<(PhysicsEventKind, ColliderHandle, ColliderHandle)>,
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

        for (kind, handle1, handle2) in self.events.drain(..) {
            events.push(PhysicsEvent::new(
                &self.rigid_body_set,
                &self.collider_set,
                kind,
                handle1,
                handle2,
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

    pub fn add_kinematic(
        &mut self,
        idx: GenerationalIndex,
        collider: MyColliderBuilder,
        position: Vec2,
    ) -> KinematicHandle {
        let collider = collider.0.user_data(idx.to_u128()).build();
        let rigid_body = RigidBodyBuilder::new_kinematic_velocity_based()
            .translation(position.into())
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );
        KinematicHandle(collider_handle, rigid_body_handle)
    }

    pub fn remove(&mut self, handle: impl Into<Handle>) {
        match handle.into() {
            Handle::Static(StaticHandle(handle)) | Handle::Sensor(SensorHandle(handle)) => {
                self.collider_set.remove(
                    handle,
                    &mut self.island_manager,
                    &mut self.rigid_body_set,
                    false,
                );
            }
            Handle::Dynamic(DynamicHandle(_, handle))
            | Handle::Kinematic(KinematicHandle(_, handle)) => {
                self.rigid_body_set.remove(
                    handle,
                    &mut self.island_manager,
                    &mut self.collider_set,
                    &mut self.joint_set,
                );
            }
        }
    }

    pub fn get_idx(&self, handle: impl Into<ColliderHandle>) -> GenerationalIndex {
        let collider = &self.collider_set[handle.into()];
        GenerationalIndex::from_u128(collider.user_data)
    }

    pub fn set_position(&mut self, handle: impl Into<ColliderHandle>, position: Vec2) {
        let body = &mut self.collider_set[handle.into()];
        body.set_translation(position.into());
    }

    pub fn get_position(&self, handle: impl Into<ColliderHandle>) -> Vec2 {
        let body = &self.collider_set[handle.into()];
        (*body.translation()).into()
    }

    pub fn get_rotation(&self, handle: impl Into<ColliderHandle>) -> f32 {
        use nalgebra::ComplexField;
        let body = &self.collider_set[handle.into()];
        body.rotation().to_polar().1
    }

    // pub fn set_linear_velocity(&mut self, handle: impl Into<RigidBodyHandle>, linvel: Vec2) {
    //     self.rigid_body_set[handle.into()].set_linvel(linvel.into(), true);
    // }

    // pub fn set_angular_velocity(&mut self, handle: impl Into<RigidBodyHandle>, angvel: f32) {
    //     self.rigid_body_set[handle.into()].set_angvel(angvel, true);
    // }

    pub fn apply_impulse(&mut self, handle: impl Into<RigidBodyHandle>, impulse: Vec2) {
        self.rigid_body_set[handle.into()].apply_impulse(impulse.into(), true);
    }

    pub fn draw_colliders(&self) {
        use nalgebra::ComplexField;

        const COLOR_STATIC: Color = Color::new(0.95, 0.0, 0.33, 0.333); // red
        const COLOR_SENSOR: Color = Color::new(0.95, 0.76, 0.0, 0.333); // yellow
        const COLOR_DYNAMIC: Color = Color::new(0.0, 0.47, 0.95, 0.333); // blue
        const COLOR_KINEMATIC: Color = Color::new(0.0, 0.95, 0.44, 0.333); // green

        for (handle, collider) in self.collider_set.iter() {
            let translation = collider.translation();

            let color = match Handle::from_collider_handle(
                &self.rigid_body_set,
                &self.collider_set,
                handle,
            ) {
                Handle::Static(_) => COLOR_STATIC,
                Handle::Sensor(_) => COLOR_SENSOR,
                Handle::Dynamic(_) => COLOR_DYNAMIC,
                Handle::Kinematic(_) => COLOR_KINEMATIC,
            };

            match collider.shape().as_typed_shape() {
                TypedShape::Ball(ball) => {
                    draw_circle(translation.x, translation.y, ball.radius, color);
                }
                TypedShape::Cuboid(cuboid) => {
                    if collider.rotation().to_polar().1 != 0. {
                        panic!("drawing rotated rectangles is unsupported");
                    }
                    let size = cuboid.half_extents * 2.;
                    let translation = translation - cuboid.half_extents;
                    draw_rectangle(translation.x, translation.y, size.x, size.y, color);
                }
                _ => panic!("drawing shape is unsupported"),
            }
        }
    }
}

pub enum PhysicsEventKind {
    IntersectStart,
    IntersectEnd,
    ContactStart,
    ContactEnd,
}

pub struct PhysicsEvent {
    pub kind: PhysicsEventKind,
    pub collider1: Handle,
    pub collider2: Handle,
}

impl PhysicsEvent {
    fn new(
        rigid_body_set: &RigidBodySet,
        collider_set: &ColliderSet,
        kind: PhysicsEventKind,
        mut handle1: ColliderHandle,
        mut handle2: ColliderHandle,
    ) -> Self {
        let datum1 = collider_set[handle1].user_data;
        let datum2 = collider_set[handle2].user_data;

        // ensure the event pair is in a consistent order every time
        if datum2 < datum1 {
            std::mem::swap(&mut handle1, &mut handle2);
        }

        PhysicsEvent {
            kind,
            collider1: Handle::from_collider_handle(rigid_body_set, collider_set, handle1),
            collider2: Handle::from_collider_handle(rigid_body_set, collider_set, handle2),
        }
    }
}

// Despite being single-threaded Rapier2d requires Sync
// (see: https://github.com/dimforge/rapier/issues/253)
struct RawEventCollector<'a>(
    Mutex<&'a mut Vec<(PhysicsEventKind, ColliderHandle, ColliderHandle)>>,
);

impl<'a> EventHandler for RawEventCollector<'a> {
    fn handle_intersection_event(&self, event: IntersectionEvent) {
        let a = event.collider1;
        let b = event.collider2;
        let kind = if event.intersecting {
            PhysicsEventKind::IntersectStart
        } else {
            PhysicsEventKind::IntersectEnd
        };
        self.0.lock().unwrap().push((kind, a, b));
    }

    fn handle_contact_event(&self, event: ContactEvent, _pair: &ContactPair) {
        let mut events = self.0.lock().unwrap();
        match event {
            ContactEvent::Started(a, b) => events.push((PhysicsEventKind::ContactStart, a, b)),
            ContactEvent::Stopped(a, b) => events.push((PhysicsEventKind::ContactEnd, a, b)),
        }
    }
}

pub struct MyColliderBuilder(ColliderBuilder);

impl MyColliderBuilder {
    pub fn intersection_events(mut self) -> Self {
        self.0.active_events |= ActiveEvents::INTERSECTION_EVENTS;
        self
    }

    pub fn contact_events(mut self) -> Self {
        self.0.active_events |= ActiveEvents::CONTACT_EVENTS;
        self
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
pub struct KinematicHandle(ColliderHandle, RigidBodyHandle);

#[derive(Clone, Copy)]
pub enum Handle {
    Static(StaticHandle),
    Sensor(SensorHandle),
    Dynamic(DynamicHandle),
    Kinematic(KinematicHandle),
}

impl Handle {
    fn from_collider_handle(
        rigid_body_set: &RigidBodySet,
        collider_set: &ColliderSet,
        collider_handle: ColliderHandle,
    ) -> Self {
        let collider = &collider_set[collider_handle];
        if let Some(rigid_body_handle) = collider.parent() {
            let rigid_body = &rigid_body_set[rigid_body_handle];
            match rigid_body.body_type() {
                RigidBodyType::Dynamic => {
                    Handle::Dynamic(DynamicHandle(collider_handle, rigid_body_handle))
                }
                RigidBodyType::KinematicVelocityBased => {
                    Handle::Kinematic(KinematicHandle(collider_handle, rigid_body_handle))
                }
                _ => panic!(),
            }
        } else if collider.is_sensor() {
            Handle::Sensor(SensorHandle(collider_handle))
        } else {
            Handle::Static(StaticHandle(collider_handle))
        }
    }
}

impl From<Handle> for ColliderHandle {
    fn from(handle: Handle) -> ColliderHandle {
        match handle {
            Handle::Static(handle) => handle.into(),
            Handle::Sensor(handle) => handle.into(),
            Handle::Dynamic(handle) => handle.into(),
            Handle::Kinematic(handle) => handle.into(),
        }
    }
}

impl From<StaticHandle> for ColliderHandle {
    fn from(handle: StaticHandle) -> ColliderHandle {
        handle.0
    }
}

impl From<SensorHandle> for ColliderHandle {
    fn from(handle: SensorHandle) -> ColliderHandle {
        handle.0
    }
}

impl From<DynamicHandle> for ColliderHandle {
    fn from(handle: DynamicHandle) -> ColliderHandle {
        handle.0
    }
}

impl From<KinematicHandle> for ColliderHandle {
    fn from(handle: KinematicHandle) -> ColliderHandle {
        handle.0
    }
}

impl From<DynamicHandle> for RigidBodyHandle {
    fn from(handle: DynamicHandle) -> RigidBodyHandle {
        handle.1
    }
}

impl From<KinematicHandle> for RigidBodyHandle {
    fn from(handle: KinematicHandle) -> RigidBodyHandle {
        handle.1
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

impl From<KinematicHandle> for Handle {
    fn from(handle: KinematicHandle) -> Handle {
        Handle::Kinematic(handle)
    }
}
