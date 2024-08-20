mod asteroids;
mod camera;
mod common;
mod player;
mod projectiles;
mod scenes;
mod score;
mod shooter;
mod spawner;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

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
        // RapierDebugRenderPlugin::default(),
        WorldInspectorPlugin::new(),
        bevy_tweening::TweeningPlugin,
        common::plugin,
        player::plugin,
        camera::plugin,
        shooter::plugin,
        asteroids::plugin,
        projectiles::plugin,
        scenes::plugin,
        score::plugin,
        spawner::plugin,
    ));
    app.run();
}
