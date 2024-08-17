use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use lens::ColorMaterialColorLens;

use crate::common::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Asteroid>();

    app.observe(on_asteroid_hit);
}

#[derive(Component, Default, Reflect)]
pub struct Asteroid {
    pub health: f32,
    pub depth: u32,
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub physics: PhysicsBundle,
}
impl AsteroidBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        depth: u32,
    ) -> Self {
        Self {
            asteroid: Asteroid {
                health: depth as f32 * 2.5,
                depth,
            },
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
                material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
                transform: Transform::from_xyz(750.0, 450.0, 0.0),
                ..default()
            },
            physics: PhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(radius),
                gravity: GravityScale(0.0),
                ..default()
            },
        }
    }
}

fn on_asteroid_hit(
    e_hit: Trigger<OnHitEnter>,
    mut cmds: Commands,
    mut q_asteroids: Query<(&mut Asteroid, &GlobalTransform, &Collider)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((mut asteroid, asteroid_xform, asteroid_collider)) =
        q_asteroids.get_mut(e_hit.entity())
    {
        let hit = &e_hit.event().0;
        asteroid.health -= hit.damage;

        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs_f32(0.3),
            ColorMaterialColorLens {
                start: Color::linear_rgb(1.0, 1.0, 1.0),
                end: Color::linear_rgb(1.0, 0.0, 0.0),
            },
        );

        cmds.entity(e_hit.entity())
            .insert(AssetAnimator::new(tween))
            .insert(ExternalImpulse {
                impulse: hit.dir.truncate() * 1000.0,
                ..default()
            });

        if asteroid.health <= 0.0 {
            cmds.entity(e_hit.entity()).despawn();

            if asteroid.depth > 0 {
                let mut rng = SimpleRng::default();

                let size = asteroid_collider.as_ball().unwrap().radius();
                let rng_dir = (rng.circle() * size).extend(0.0);
                let depth = asteroid.depth - 1;

                let mut bundle =
                    AsteroidBundle::new(&mut meshes, &mut materials, size / 2.0, depth);
                bundle.mesh.transform.translation = asteroid_xform.translation() + rng_dir;
                cmds.spawn(bundle);

                let mut bundle =
                    AsteroidBundle::new(&mut meshes, &mut materials, size / 2.0, depth);
                bundle.mesh.transform.translation = asteroid_xform.translation() - rng_dir;
                cmds.spawn(bundle);
            }
        }
    }
}
