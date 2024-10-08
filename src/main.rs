mod asteroids;
mod camera;
mod common;
mod player;
mod prelude;
mod projectiles;
mod scenes;
mod score;
mod shooter;
mod spawner;
mod ui;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::Wireframe2dPlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use prelude::*;

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
        HanabiPlugin,
    ))
    .add_plugins((
        common::plugin,
        player::plugin,
        camera::plugin,
        shooter::plugin,
        asteroids::plugin,
        projectiles::plugin,
        scenes::plugin,
        score::plugin,
        spawner::plugin,
        ui::plugin,
    ));
    app.run();
}
