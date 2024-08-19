use bevy::{
    color::palettes::css::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::common::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Projectile>();
    app.add_systems(PreUpdate, look_at_velocity);
    app.add_systems(
        Update,
        (
            resolve_projectile_collision,
            // debug_projectile_direction,
            // look_at_velocity,
        ),
    );
}

#[derive(Component, Default, Reflect)]
pub struct Projectile {
    damage: f32,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub name: Name,
    pub projectile: Projectile,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub lifetime: Lifetime,
    pub physics: PhysicsBundle,
    pub sensor: Sensor,
    pub physics_events: ActiveEvents,
    pub locked_axis: LockedAxes,
}

impl ProjectileBundle {
    pub fn normal_projectile(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        position: Vec2,
        velocity: Vec2,
    ) -> Self {
        let mut xform = Transform::from_translation(position.extend(0.0));
        xform.look_to(Vec3::Z, velocity.extend(0.0).normalize());
        Self {
            name: Name::new("Projectile"),
            projectile: Projectile { damage: 1.0 },
            lifetime: Lifetime::new(300.0),
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
                material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
                transform: xform,
                ..default()
            },
            physics: PhysicsBundle {
                rigidbody: RigidBody::KinematicVelocityBased,
                collider: Collider::ball(radius),
                gravity: GravityScale(0.0),
                velocity: Velocity {
                    linvel: velocity,
                    angvel: 0.0,
                },
                restitution: Restitution::coefficient(0.7),
                ..default()
            },
            locked_axis: LockedAxes::empty(),
            sensor: Sensor,
            physics_events: ActiveEvents::COLLISION_EVENTS,
            ..default()
        }
    }
}

fn resolve_projectile_collision(
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_projectiles: Query<(&Projectile, &Transform)>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Ok((proj, xform)) = q_projectiles.get(*e1) {
                hit_entity(&mut cmds, proj, xform, *e1, *e2);
            } else if let Ok((proj, xform)) = q_projectiles.get(*e2) {
                hit_entity(&mut cmds, proj, xform, *e2, *e1);
            }
        }
    }
}

fn debug_projectile_direction(
    q_projectiles: Query<(&Projectile, &Transform, &Velocity)>,
    mut gizmos: Gizmos,
) {
    for (proj, xform, vel) in q_projectiles.iter() {
        gizmos.arrow_2d(
            xform.translation.xy(),
            xform.translation.xy() + vel.linvel.normalize() * 100.0,
            GREEN,
        );
        gizmos.arrow_2d(
            xform.translation.xy(),
            xform.translation.xy() + xform.up().xy() * 100.0,
            BLUE,
        );
    }
}

fn look_at_velocity(mut q_projectiles: Query<(&Velocity, &mut Transform), With<Projectile>>) {
    for (vel, mut xform) in q_projectiles.iter_mut() {
        xform.look_to(Vec3::Z, vel.linvel.normalize().extend(0.0));
        println!("{:?}", xform.up());
    }
}

fn hit_entity(
    cmds: &mut Commands,
    projectile: &Projectile,
    xform: &Transform,
    entity_source: Entity,
    entity_target: Entity,
) {
    println!("{:?}", xform);
    let hit = HitData {
        damage: projectile.damage,
        point: xform.translation,
        dir: xform.up(),
        dealer: entity_source,
    };
    cmds.trigger_targets(OnHitEnter(hit), entity_target);
    cmds.entity(entity_source).despawn();
}
