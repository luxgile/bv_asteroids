pub mod follower;
pub use follower::*;

use std::time::Duration;

use bevy::{ecs::reflect, prelude::*};
use bevy_rapier2d::prelude::*;

use rand::{prelude::*, Rng};

pub fn plugin(app: &mut App) {
    app.register_type::<Lifetime>();
    app.add_systems(Update, process_lifetimes);

    app.add_event::<OnCollisionEnter>().add_event::<OnHit>();
    app.observe(apply_destroy_on_death);

    app.register_type::<PickUp>();
    app.register_type::<PickUpReceiver>();
    app.add_event::<OnPickedUp>();
    app.add_systems(Update, (detect_pickups, attract_pickups));

    app.add_plugins(follower::plugin);
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
pub struct OnHit(pub HitData);

#[derive(Event)]
pub struct OnDeath(pub HitData);

#[derive(Component, Default)]
pub struct DestroyOnDeath;

fn apply_destroy_on_death(
    e_death: Trigger<OnDeath>,
    mut cmds: Commands,
    q_tags: Query<&DestroyOnDeath>,
) {
    if q_tags.get(e_death.entity()).is_ok() {
        cmds.entity(e_death.entity()).despawn();
    }
}

#[derive(Debug, Clone, Copy)]
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
#[reflect(Component)]
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
            cmds.entity(entity).despawn_recursive();
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum PickUp {
    #[default]
    Idle,
    Picking,
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PickUpReceiver {
    pub check_timer: Timer,
    pub chase_distance: f32,
    pub pick_distance: f32,
    pub pick_speed: f32,
    pub entities_picked: Vec<Entity>,
}
#[derive(Event)]
pub struct OnPickedUp {
    receiver_entity: Entity,
}

fn detect_pickups(
    r_rapier: Res<RapierContext>,
    r_time: Res<Time>,
    mut q_receivers: Query<(&mut PickUpReceiver, &GlobalTransform)>,
    mut q_pickups: Query<&mut PickUp>,
) {
    for (mut receiver, xform) in q_receivers.iter_mut() {
        if receiver.check_timer.tick(r_time.delta()).finished() {
            let circle = Collider::ball(receiver.chase_distance);
            r_rapier.intersections_with_shape(
                xform.translation().xy(),
                0.0,
                &circle,
                QueryFilter::default(),
                |entity| {
                    if let Ok(mut pickup) = q_pickups.get_mut(entity) {
                        match *pickup {
                            PickUp::Idle => {
                                *pickup = PickUp::Picking;
                                receiver.entities_picked.push(entity);
                            }
                            PickUp::Picking => return true,
                        }
                    }
                    true
                },
            );
        }
    }
}

fn attract_pickups(
    mut cmds: Commands,
    r_time: Res<Time>,
    mut q_receivers: Query<(Entity, &mut PickUpReceiver, &GlobalTransform)>,
    mut q_pickups: Query<(&mut Transform), With<PickUp>>,
) {
    for (receiver_entity, mut receiver, xform) in q_receivers.iter_mut() {
        let pick_distance = receiver.pick_distance;
        let pick_speed = receiver.pick_speed;
        let mut entities_to_remove = Vec::new();
        for (i, e) in receiver.entities_picked.iter_mut().enumerate() {
            if let Ok(mut pickup) = q_pickups.get_mut(*e) {
                let pickup_pos = pickup.translation;
                pickup.translation += (xform.translation() - pickup_pos).normalize()
                    * pick_speed
                    * r_time.delta_seconds();

                if xform.translation().distance(pickup.translation) <= pick_distance {
                    entities_to_remove.push(*e);
                    cmds.entity(*e).despawn();
                    cmds.trigger_targets(OnPickedUp { receiver_entity }, *e);
                }
            }
        }

        for i in entities_to_remove {
            receiver.entities_picked.retain(|x| *x != i);
        }
    }
}
