use crate::prelude::*;

#[derive(Component, Default)]
pub struct HittableButton {
    pub distance: f32,
}

pub trait SpawnHittableButtonExt {
    fn spawn_hittable_button(
        &mut self,
        r_assets: &Res<AssetServer>,
        r_effects: &mut ResMut<Assets<EffectAsset>>,
        image: Handle<Image>,
        text: String,
    ) -> Entity;
}
impl<'w, 's> SpawnHittableButtonExt for Commands<'w, 's> {
    fn spawn_hittable_button(
        &mut self,
        r_assets: &Res<AssetServer>,
        r_effects: &mut ResMut<Assets<EffectAsset>>,
        image: Handle<Image>,
        text: String,
    ) -> Entity {
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
        gradient.add_key(1.0, Vec4::splat(0.));

        let writer = ExprWriter::new();

        let init_pos = SetPositionCircleModifier {
            axis: writer.lit(Vec3::Z).expr(),
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(100.0).expr(),
            dimension: ShapeDimension::Surface,
        };

        let init_vel = SetVelocityCircleModifier {
            axis: writer.lit(Vec3::Z).expr(),
            center: writer.lit(Vec3::ZERO).expr(),
            speed: (writer.lit(300.0)
                + writer.rand(ValueType::Scalar(ScalarType::Float)) * writer.lit(300.0))
            .expr(),
        };

        let size = SetSizeModifier {
            size: CpuValue::Single(Vec2::ONE * 20.0),
        };

        let lifetime = writer.lit(0.2).expr(); // literal value "10.0"
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let mut spawner = Spawner::once(25.0.into(), false);
        let effect = EffectAsset::new(vec![32768], spawner, writer.finish())
            .with_name("MyEffect")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient })
            .render(size);

        self.spawn((
            HittableButton { distance: 500.0 },
            ParticleEffectBundle {
                effect: ParticleEffect::new(r_effects.add(effect)),
                transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
                ..default()
            },
            PhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                gravity: GravityScale(0.0),
                velocity: Velocity::default(),
                collider: Collider::cuboid(120.0, 120.0),
                mass: ColliderMassProperties::Mass(1.0),
                ..default()
            },
        ))
        .with_children(|e| {
            e.spawn(SpriteBundle {
                texture: image,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                ..default()
            });

            e.spawn(SpriteBundle {
                texture: r_assets.load("ui/button_outline.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(120.0, 120.0)),
                    ..default()
                },
                ..default()
            });

            e.spawn(Text2dBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: 80.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, -140.0, 0.0),
                ..default()
            });
        })
        .id()
    }
}
