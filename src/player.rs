use std::time::Duration;

use crate::common::*;
use crate::scenes::GameStates;
use crate::shooter::*;
use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_systems(
        Update,
        (
            player_movement,
            player_look_at_mouse,
            player_input_shooting,
            player_death_touch,
        )
            .run_if(in_state(GameStates::InGame)),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

pub struct SpawnPlayer;
impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}
fn spawn_player(
    spawn: In<SpawnPlayer>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn((
        Name::new("Player"),
        StateScoped(GameStates::InGame),
        Player,
        Shooter {
            shoot_delay: 0.4,
            ..default()
        },
        PickUpReceiver {
            check_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            chase_distance: 500.0,
            pick_distance: 50.0,
            pick_speed: 300.0,
            ..default()
        },
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 50.0),
                Vec2::new(-50.0, -50.0),
                Vec2::new(50.0, -50.0),
            ))),
            material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
            ..default()
        },
        PhysicsBundle {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::triangle(
                Vect::new(0.0, 50.0),
                Vect::new(-50.0, -50.0),
                Vect::new(50.0, -50.0),
            ),
            damping: Damping {
                linear_damping: 1.0,
                ..default()
            },
            gravity: GravityScale(0.0),
            restitution: Restitution::new(0.7),
            ..default()
        },
    ));
}

#[derive(Component, Default)]
pub struct KillPlayerOnTouch;

#[derive(Event, Default)]
pub struct OnPlayerDeath;

fn player_movement(
    mut q_players: Query<&mut Velocity, With<Player>>,
    kbd: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if q_players.is_empty() {
        return;
    }

    let mut player = q_players.single_mut();
    let mut force = Vec2::ZERO;
    if kbd.pressed(KeyCode::KeyW) {
        force += Vec2::Y;
    }
    if kbd.pressed(KeyCode::KeyS) {
        force -= Vec2::Y;
    }
    if kbd.pressed(KeyCode::KeyD) {
        force += Vec2::X;
    }
    if kbd.pressed(KeyCode::KeyA) {
        force -= Vec2::X;
    }
    player.linvel += force * 2000.0 * time.delta_seconds();
    player.linvel = player.linvel.clamp_length(0.0, 1250.0);
}

fn player_look_at_mouse(
    mut q_players: Query<&mut Transform, With<Player>>,
    q_wnd: Query<&Window>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
) {
    if q_players.is_empty() {
        return;
    }
    let (camera, camera_xform) = q_cam.single();
    let window = q_wnd.single();
    let mut player = q_players.single_mut();

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_xform, cursor))
    {
        let player_pos = player.translation;
        let player_forward = player.forward().as_vec3();
        player.look_at(player_pos + player_forward, pos.extend(0.0) - player_pos);
    }
}

fn player_input_shooting(
    mut q_players: Query<&mut Shooter, With<Player>>,
    kbd: Res<ButtonInput<MouseButton>>,
) {
    if q_players.is_empty() {
        return;
    }

    let mut player = q_players.single_mut();
    player.enabled = kbd.pressed(MouseButton::Left);
}

fn player_death_touch(
    mut cmds: Commands,
    mut q_players: Query<Entity, With<Player>>,
    q_deathtouch: Query<Entity, With<KillPlayerOnTouch>>,
    r_rapier: Res<RapierContext>,
) {
    let player = q_players.single();
    for pair in r_rapier.contact_pairs_with(player) {
        let other = if pair.collider1() == player {
            pair.collider2()
        } else {
            pair.collider1()
        };
        if q_deathtouch.get(other).is_ok() {
            cmds.trigger_targets(OnPlayerDeath, player);
            println!("Holy fuck the player is dead!");
        }
    }
}
