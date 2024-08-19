use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::{prelude::*, Rng};

pub fn plugin(app: &mut App) {
    app.register_type::<Lifetime>();
    app.add_systems(Update, process_lifetimes);
    app.add_event::<OnCollisionEnter>();
    app.add_event::<OnHitEnter>();
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub damping: Damping,
    pub mass: ColliderMassProperties,
}

#[derive(Event)]
pub struct OnCollisionEnter;

#[derive(Event)]
pub struct OnHitEnter(pub HitData);

#[derive(Debug)]
pub struct HitData {
    pub point: Vec3,
    pub dir: Dir3,
    pub dealer: Entity,
    pub damage: f32,
}
impl Default for HitData {
    fn default() -> Self {
        Self {
            dealer: Entity::PLACEHOLDER,
            ..default()
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct Lifetime(Timer);
impl Lifetime {
    pub fn new(time: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(time), TimerMode::Once))
    }
}

fn process_lifetimes(
    mut cmds: Commands,
    time: Res<Time>,
    mut q_lifetimes: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in q_lifetimes.iter_mut() {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            cmds.entity(entity).despawn();
        }
    }
}

pub trait RngSampler {
    fn value(&mut self) -> f32;
    fn value_one(&mut self) -> f32 {
        (self.value() - 0.5) * 2.0
    }
    fn value_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.value()
    }
    fn circle(&mut self) -> Vec2 {
        Vec2::new(self.value_one(), self.value_one()).normalize()
    }
    fn sphere(&mut self) -> Vec3 {
        Vec3::new(self.value_one(), self.value_one(), self.value_one()).normalize()
    }
}

#[derive(Default)]
pub struct SimpleRng {
    rng: ThreadRng,
}
impl RngSampler for SimpleRng {
    fn value(&mut self) -> f32 {
        self.rng.gen()
    }
}
