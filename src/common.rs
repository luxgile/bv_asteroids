use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Lifetime>();
    app.add_systems(Update, process_lifetimes);
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub damping: Damping,
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
