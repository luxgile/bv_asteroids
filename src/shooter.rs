use crate::projectiles::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Shooter>();

    app.add_systems(Update, (shooter_fire));
}

#[derive(Component, Default, Reflect)]
pub struct Shooter {
    pub enabled: bool,
    pub shoot_timer: f32,
    pub shoot_delay: f32,
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
            let projectile = ProjectileBundle::normal_projectile(
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
