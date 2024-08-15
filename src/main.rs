mod common;
mod projectiles;
use common::*;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Shooter {
    enabled: bool,
    shoot_timer: f32,
    shoot_delay: f32,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        // Need to config the render plugin to avoid spamming error messages.
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            ..default()
        }),
        Wireframe2dPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        common::plugin,
        projectiles::plugin,
        // RapierDebugRenderPlugin::default(),
    ));
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            player_movement,
            player_look_at_mouse,
            player_input_shooting,
            shooter_fire,
        ),
    );
    app.run();
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 5.0;
    cmds.spawn(camera);

    cmds.spawn(Player)
        .insert(Shooter {
            shoot_delay: 0.4,
            ..default()
        })
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 50.0),
                Vec2::new(-50.0, -50.0),
                Vec2::new(50.0, -50.0),
            ))),
            material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::triangle(
            Vect::new(0.0, 50.0),
            Vect::new(-50.0, -50.0),
            Vect::new(50.0, -50.0),
        ))
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 0.0,
        })
        .insert(GravityScale(0.0))
        .insert(Restitution::coefficient(0.7));

    cmds.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(200.0))),
        material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_xyz(750.0, 450.0, 0.0),
        ..default()
    })
    .insert((RigidBody::Fixed, Collider::ball(200.0)));
}

fn player_movement(
    mut q_players: Query<&mut Velocity, With<Player>>,
    kbd: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if q_players.is_empty() {
        return;
    };

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
    let mut player = q_players.single_mut();
    player.enabled = kbd.pressed(MouseButton::Left);
}

fn shooter_fire(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut q_shooters: Query<(&GlobalTransform, &mut Shooter)>,
) {
    for (transform, mut shooter) in q_shooters.iter_mut() {
        if !shooter.enabled {
            return;
        }

        shooter.shoot_timer += time.delta_seconds();
        while shooter.shoot_timer >= shooter.shoot_delay {
            shooter.shoot_timer -= shooter.shoot_delay;

            let radius = 10.0;
            let spawn_velocity = (transform.up().as_vec3() * 2000.0).xy();
            let spawn_position = (transform.translation() + transform.up().as_vec3() * 75.0).xy();
            let projectile = projectiles::ProjectileBundle::normal_projectile(
                &mut meshes,
                &mut materials,
                radius,
                spawn_position,
                spawn_velocity,
            );
            cmds.spawn(projectile);
        }
    }
}
