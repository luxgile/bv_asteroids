use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::common::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Asteroid>();
}

#[derive(Component, Default, Reflect)]
pub struct Asteroid;

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
            asteroid: Asteroid,
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
