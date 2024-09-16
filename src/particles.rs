//! To use anything in this module you need to:
//! 1. cargo add bevy_particle_systems
//! 2. .add_plugins(ParticleSystemPlugin)

use bevy::prelude::*;
use bevy_particle_systems::*;

/// Trail of particles.
///
/// After a commands.spawn() do...
///    .with_children(|builder| {
///        builder.spawn(particle_trail_bundle());
///    });
pub fn particle_trail_bundle() -> (ParticleSystemBundle, Playing) {
    (
        ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 1000,
                emitter_shape: CircleSegment {
                    radius: JitteredValue::centered_range(0.0..26.0),
                    ..Default::default()
                }
                .into(),
                spawn_rate_per_second: 125.0.into(),
                initial_speed: JitteredValue::jittered(5.0, 0.0..10.0),
                lifetime: JitteredValue::jittered(2.0, -1.0..1.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::srgba(1.0, 1.0, 0.0, 0.6), 0.0),
                    CurvePoint::new(Color::srgba(1.0, 0.0, 0.0, 0.0), 1.0),
                ])),
                looping: true,
                system_duration_seconds: 10.0,
                space: ParticleSpace::World,
                scale: 4.0.into(),
                rotation_speed: 2.0.into(),
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -0.5),
            ..ParticleSystemBundle::default()
        },
        Playing,
    )
}

/// Spawn a particle poof at the transform's translation, adjusted to a Z of 10.0.
pub fn spawn_particle_poof(commands: &mut Commands, transform: &Transform) {
    let mut transform = *transform;
    transform.translation.z = 10.0;
    commands.spawn((
        ParticleSystemBundle {
            transform,
            particle_system: ParticleSystem {
                spawn_rate_per_second: 0.0.into(),
                emitter_shape: EmitterShape::CircleSegment(CircleSegment {
                    radius: JitteredValue::centered_range(0.0..26.0),
                    ..Default::default()
                }),
                max_particles: 1_000,
                initial_speed: (0.0..130.0).into(),
                scale: 4.0.into(),
                lifetime: JitteredValue::jittered(1.0, -0.5..0.5),
                velocity_modifiers: vec![
                    VelocityModifier::Drag(0.002.into()),
                    VelocityModifier::Vector(Vec3::new(0.0, -200.0, 0.0).into()),
                ],
                color: (Color::srgba(1.0, 1.0, 0.0, 0.8)..Color::srgba(1.0, 0.0, 0.0, 0.0)).into(),
                bursts: vec![ParticleBurst {
                    time: 0.0,
                    count: 300,
                }],
                ..ParticleSystem::oneshot()
            },
            ..default()
        },
        Playing,
    ));
}
