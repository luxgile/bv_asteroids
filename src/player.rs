use crate::common::*;
use crate::scenes::GameStates;
use crate::shooter::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_systems(
        Update,
        (player_movement, player_look_at_mouse, player_input_shooting)
            .run_if(in_state(GameStates::InGame)),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    shooter: Shooter,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    physics: PhysicsBundle,
}
impl PlayerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            player: Player,
            shooter: Shooter {
                shoot_delay: 0.4,
                ..default()
            },
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                    Vec2::new(0.0, 50.0),
                    Vec2::new(-50.0, -50.0),
                    Vec2::new(50.0, -50.0),
                ))),
                material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
                ..default()
            },
            physics: PhysicsBundle {
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
        }
    }
}

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
