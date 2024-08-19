use std::time::Duration;

use bevy::{
    color::palettes::css::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use lens::ColorMaterialColorLens;

use crate::{common::*, score::Score};

pub fn plugin(app: &mut App) {
    app.register_type::<Asteroid>();

    app.observe(on_asteroid_hit).observe(on_asteroid_death);
}

#[derive(Component, Default, Reflect)]
pub struct Asteroid {
    pub health: f32,
    pub depth: u32,
}

#[derive(Bundle, Default)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub physics: PhysicsBundle,
    pub destroy_on_death: DestroyOnDeath,
    pub score: Score,
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
            score: Score(depth),
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
                material: materials.add(Color::from(BROWN)),
                transform: Transform::from_xyz(750.0, 450.0, 0.0),
                ..default()
            },
            physics: PhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(radius),
                gravity: GravityScale(0.0),
                mass: ColliderMassProperties::Mass(1.0 * depth as f32),
                ..default()
            },
            ..default()
        }
    }
}

fn on_asteroid_hit(
    e_hit: Trigger<OnHit>,
    mut cmds: Commands,
    mut q_asteroids: Query<(&mut Asteroid, &GlobalTransform, &Collider, &Velocity)>,
) {
    if let Ok((mut asteroid, asteroid_xform, asteroid_collider, vel)) =
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
                impulse: hit.dir.xy() * 100.0,
                ..default()
            });

        if asteroid.health <= 0.0 {
            cmds.trigger_targets(OnDeath(*hit), e_hit.entity());
        }
    }
}

fn on_asteroid_death(
    e_death: Trigger<OnDeath>,
    mut cmds: Commands,
    mut q_asteroids: Query<(&mut Asteroid, &GlobalTransform, &Collider, &Velocity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((mut asteroid, asteroid_xform, asteroid_collider, vel)) =
        q_asteroids.get_mut(e_death.entity())
    {
        if asteroid.depth > 0 {
            let mut rng = SimpleRng::default();

            let size = asteroid_collider.as_ball().unwrap().radius();
            let rng_dir = (rng.circle() * size).extend(0.0);
            let depth = asteroid.depth - 1;

            let mut bundle = AsteroidBundle::new(&mut meshes, &mut materials, size / 2.0, depth);
            bundle.mesh.transform.translation = asteroid_xform.translation() + rng_dir;
            bundle.physics.velocity.linvel =
                Vec2::from_angle(f32::to_radians(-45.0 / (4.0 - asteroid.depth as f32)))
                    .rotate(vel.linvel);
            cmds.spawn(bundle);

            let mut bundle = AsteroidBundle::new(&mut meshes, &mut materials, size / 2.0, depth);
            bundle.mesh.transform.translation = asteroid_xform.translation() - rng_dir;
            bundle.physics.velocity.linvel =
                Vec2::from_angle(f32::to_radians(45.0 / (4.0 - asteroid.depth as f32)))
                    .rotate(vel.linvel);
            cmds.spawn(bundle);
        }
    }
}
