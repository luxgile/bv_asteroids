use std::time::Duration;

use crate::{asteroids::*, common::*, player::Player};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<AsteroidSpawner>();

    app.add_systems(Update, run_asteroid_spawner);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AsteroidSpawner {
    pub timer: Timer,
}

fn run_asteroid_spawner(
    mut cmds: Commands,
    q_wnd: Query<&Window>,
    q_player: Query<(&GlobalTransform), With<Player>>,
    mut q_spawners: Query<&mut AsteroidSpawner>,
    r_time: Res<Time>,
) {
    let wnd = q_wnd.single();
    let player = q_player.single();
    let mut rng = SimpleRng::default();

    for mut spawner in q_spawners.iter_mut() {
        spawner.timer.tick(r_time.delta());
        if spawner.timer.finished() {
            let spawn_pos = player.translation().xy() + rng.circle() * wnd.size().x * 2.0;
            cmds.add(SpawnAsteroid {
                depth: 2,
                position: spawn_pos,
                velocity: (player.translation().xy() - spawn_pos).normalize()
                    * rng.value_range(100.0, 350.0),
            });
        }
    }
}
