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
    pub health: u32,
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
    ) -> Self {
        Self {
            asteroid: Asteroid { health: 10 },
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(200.0))),
                material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
                transform: Transform::from_xyz(750.0, 450.0, 0.0),
                ..default()
            },
            physics: PhysicsBundle {
                rigidbody: RigidBody::Fixed,
                collider: Collider::ball(200.0),
                ..default()
            },
        }
    }
}

fn on_asteroid_hit(
    e_hit: Trigger<OnHitEnter>,
    mut cmds: Commands,
    mut q_asteroids: Query<&mut Asteroid>,
) {
    if let Ok(mut asteroid) = q_asteroids.get_mut(e_hit.entity()) {
        asteroid.health -= 1;

        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs_f32(0.3),
            ColorMaterialColorLens {
                start: Color::linear_rgb(1.0, 1.0, 1.0),
                end: Color::linear_rgb(1.0, 0.0, 0.0),
            },
        );
        cmds.entity(e_hit.entity())
            .insert(AssetAnimator::new(tween));

        if asteroid.health == 0 {
            cmds.entity(e_hit.entity()).despawn();
        }
    }
}
