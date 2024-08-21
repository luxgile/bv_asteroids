use std::ops::Range;

use bevy::{
    color::palettes::css::YELLOW,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{common::*, scenes::GameStates};

pub fn plugin(app: &mut App) {
    app.insert_resource(PlayerCurrency(0));
    app.register_type::<PlayerCurrency>();
    app.register_type::<Money>();

    app.observe(on_pickup_money);
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerCurrency(pub u32);

/// Amount of money to be dropped on death.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MoneyDrop(pub u32);

/// Amount of money to be obtained when picked up.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Money(pub u32);

pub struct SpawnMoney {
    pub money: u32,
    pub position: Vec2,
    pub radial_force: Range<f32>,
}
impl Command for SpawnMoney {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_money);
    }
}
fn spawn_money(
    spawn: In<SpawnMoney>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = SimpleRng::default();
    for i in 0..spawn.0.money {
        cmds.spawn((
            Money(1),
            StateScoped(GameStates::InGame),
            PickUp::Idle,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::new(20.0, 20.0)))),
                material: materials.add(Color::from(YELLOW)),
                transform: Transform::from_translation(spawn.position.extend(0.0)),
                ..default()
            },
            RigidBody::Dynamic,
            Sensor,
            Collider::cuboid(10.0, 10.0),
            Damping {
                linear_damping: 1.0,
                ..default()
            },
            GravityScale(0.0),
            Velocity {
                linvel: rng.circle()
                    * rng.value_range(spawn.radial_force.start, spawn.radial_force.end),
                ..default()
            },
        ));
    }
}

fn on_pickup_money(
    e_pickup: Trigger<OnPickedUp>,
    q_score: Query<&Money>,
    mut r_score: ResMut<PlayerCurrency>,
) {
    if let Ok(score) = q_score.get(e_pickup.entity()) {
        r_score.0 += score.0;
        println!("New score: {:?}", r_score.0);
    }
}
