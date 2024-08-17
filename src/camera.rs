use bevy::prelude::*;

use crate::player::Player;

pub fn plugin(app: &mut App) {
    app.register_type::<PlayerCamera>();

    app.add_systems(Update, camera_follow_player);
}

#[derive(Component, Default, Reflect)]
pub struct PlayerCamera {
    pub smoothness: f32,
}

#[derive(Bundle, Default)]
pub struct PlayerCameraBundle {
    pub player_camera: PlayerCamera,
    pub camera: Camera2dBundle,
}
impl PlayerCameraBundle {
    pub fn new() -> Self {
        Self {
            player_camera: PlayerCamera { smoothness: 0.97 },
            ..default()
        }
    }
}

fn camera_follow_player(
    q_player: Query<&GlobalTransform, With<Player>>,
    mut q_camera: Query<(&mut Transform, &PlayerCamera)>,
) {
    if q_player.is_empty() {
        return;
    }

    let player_xform = q_player.single();
    for (mut camera_xform, camera) in q_camera.iter_mut() {
        camera_xform.translation = player_xform
            .translation()
            .lerp(camera_xform.translation, camera.smoothness);
    }
}
