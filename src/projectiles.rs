use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::common::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Projectile>();
    app.add_systems(Update, resolve_projectile_collision);
}

#[derive(Component, Default, Reflect)]
pub struct Projectile {
    damage: f32,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    projectile: Projectile,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    lifetime: Lifetime,
    physics: PhysicsBundle,
    sensor: Sensor,
    physics_events: ActiveEvents,
}

impl ProjectileBundle {
    pub fn normal_projectile(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        position: Vec2,
        velocity: Vec2,
    ) -> Self {
        Self {
            projectile: Projectile { damage: 10.0 },
            lifetime: Lifetime::new(3.0),
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
                material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            physics: PhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(radius),
                gravity: GravityScale(0.0),
                velocity: Velocity {
                    linvel: velocity,
                    ..default()
                },
                restitution: Restitution::coefficient(0.7),
                ..default()
            },
            sensor: Sensor,
            physics_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

fn resolve_projectile_collision(
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_projectiles: Query<&Projectile>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            println!("{:?}", event);
            let projectile = q_projectiles.get(*e1).or(q_projectiles.get(*e2));
            if projectile.is_ok() {
                println!("HIT!");
                cmds.entity(*e1).despawn();
                cmds.entity(*e2).despawn();
            }
        }
    }
}
