use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::css::*,
    ecs::{reflect, system::RunSystemOnce, world::Command},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use lens::ColorMaterialColorLens;
use rand::Rng;

use crate::{
    common::*,
    player::KillPlayerOnTouch,
    scenes::GameStates,
    score::{MoneyDrop, SpawnMoney},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Asteroid>();

    app.observe(on_asteroid_hit).observe(on_asteroid_death);
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Asteroid {
    pub health: f32,
    pub depth: u32,
}

pub struct SpawnAsteroid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub depth: u32,
}
impl Command for SpawnAsteroid {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_asteroid);
    }
}
fn spawn_asteroid(
    spawn: In<SpawnAsteroid>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let depth = spawn.depth;
    let health = (depth + 1) as f32 * 2.5;
    let radius = (depth + 1) as f32 * 100.0;
    cmds.spawn((
        Name::new(format!("Asteroid - {:?}", depth)),
        StateScoped(GameStates::Match),
        Asteroid { health, depth },
        MoneyDrop(depth + 1),
        KillPlayerOnTouch,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
            material: materials.add(Color::from(GREY)),
            transform: Transform::from_translation(spawn.position.extend(0.0)),
            ..default()
        },
        PhysicsBundle {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(radius - 10.0), // Adding some room to the collider for the player.
            gravity: GravityScale(0.0),
            mass: ColliderMassProperties::Mass(1.0 * depth as f32),
            velocity: Velocity {
                linvel: spawn.velocity,
                ..default()
            },
            ..default()
        },
        DestroyOnDeath,
    ));
}
fn generate_asteroid_shape(radius: f32) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let resolution = 32;
    let delta = 10.0;
    let angle_dt = PI / resolution as f32;
    let mut vertices = Vec::with_capacity(resolution);
    for i in 0..resolution {
        let angle = i as f32 * angle_dt;
        vertices.push(Vec2::from_angle(angle) * (radius + rng.gen_range(-delta..delta)));
    }
    vertices
}

fn on_asteroid_hit(
    e_hit: Trigger<OnHit>,
    mut cmds: Commands,
    mut q_asteroids: Query<&mut Asteroid>,
) {
    if let Ok(mut asteroid) = q_asteroids.get_mut(e_hit.entity()) {
        let hit = &e_hit.event().0;
        asteroid.health -= hit.damage;

        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs_f32(0.3),
            ColorMaterialColorLens {
                start: WHITE.into(),
                end: GREY.into(),
            },
        );

        cmds.entity(e_hit.entity())
            .insert(AssetAnimator::new(tween))
            .insert(ExternalImpulse {
                impulse: hit.dir.xy() * 10.0,
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
    mut q_asteroids: Query<(
        &mut Asteroid,
        &GlobalTransform,
        &Collider,
        &Velocity,
        Option<&MoneyDrop>,
    )>,
) {
    if let Ok((mut asteroid, asteroid_xform, asteroid_collider, vel, money_drop)) =
        q_asteroids.get_mut(e_death.entity())
    {
        if let Some(money) = money_drop {
            cmds.add(SpawnMoney {
                money: money.0,
                position: asteroid_xform.translation().xy(),
                radial_force: 150.0..300.0,
            });
        }

        if asteroid.depth > 0 {
            let size = asteroid_collider.as_ball().unwrap().radius();
            let spawn_offset = Vec2::from_angle(PI / 2.0).rotate(vel.linvel.normalize()) * size;
            let depth = asteroid.depth - 1;

            cmds.add(SpawnAsteroid {
                position: asteroid_xform.translation().xy() - spawn_offset,
                velocity: vel.linvel - spawn_offset.normalize() * 50.0,
                depth,
            });

            cmds.add(SpawnAsteroid {
                position: asteroid_xform.translation().xy() + spawn_offset,
                velocity: vel.linvel + spawn_offset.normalize() * 50.0,
                depth,
            });
        }
    }
}
