use crate::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HittableButton;

#[derive(Bundle, Default)]
pub struct HittableButtonBundle {
    name: Name,
    button: HittableButton,
    sprite: SpriteBundle,
}
