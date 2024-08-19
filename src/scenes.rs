use crate::*;
use bevy::prelude::*;
use camera::PlayerCameraBundle;
use common::*;

pub fn plugin(app: &mut App) {
    app.register_type::<GameStates>();

    app.insert_state(GameStates::InGame);

    app.add_systems(OnEnter(GameStates::InGame), ingame_setup);
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Hash, States, Reflect)]
pub enum GameStates {
    Menu,
    #[default]
    InGame,
}

fn ingame_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut player_camera = PlayerCameraBundle::new();
    player_camera.camera.projection.scale = 5.0;
    cmds.spawn(player_camera);

    let player = player::PlayerBundle::new(&mut meshes, &mut materials);
    cmds.spawn(player);

    let mut rng = SimpleRng::default();
    let mut asteroid = asteroids::AsteroidBundle::new(&mut meshes, &mut materials, 200.0, 2);
    asteroid.mesh.transform.translation = Vec3::new(750.0, 450.0, 0.0);
    // asteroid.physics.velocity.linvel = rng.circle() * rng.value_range(100.0, 500.0);
    cmds.spawn(asteroid);
}
