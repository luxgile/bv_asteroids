use std::time::Duration;

use crate::*;
use asteroids::SpawnAsteroid;
use bevy::prelude::*;
use camera::PlayerCameraBundle;
use common::*;
use score::SpawnMoney;
use spawner::AsteroidSpawner;

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

    cmds.spawn(AsteroidSpawner {
        timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Repeating),
    });

    cmds.add(SpawnMoney {
        money: 10,
        position: Vec2::new(750.0, 500.0),
        radial_force: 100.0..250.0,
    })
}
