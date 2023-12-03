pub const G: f32 = 10.0;
use bevy::prelude::*;
#[derive(Component)]
pub struct Mass {
    pub mass: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct OrbitalBody {
    pub velocity: Vec3,
}

pub fn apply_gravity(
    time: Res<Time>,
    mut boddies: Query<(Entity, &mut OrbitalBody, &Transform, &Mass)>,
    mut gravity_boddies: Query<(Entity, &Mass, &Transform)>,
) {
    let delta = time.delta_seconds();
    for (id, mut orbital, body_transform, body_mass) in boddies.iter_mut() {
        let mut force = Vec3::ZERO;
        for (id2, Mass { mass, radius }, gravity_transform) in gravity_boddies.iter_mut() {
            if id == id2 {
                continue;
            }
            let gravity = mass
                / body_transform
                    .translation
                    .distance_squared(gravity_transform.translation)
                    .max(radius * radius + body_mass.radius * body_mass.radius)
                * (gravity_transform.translation - body_transform.translation).normalize();

            force += gravity
        }
        orbital.velocity += delta * force * G / body_mass.mass;
    }
}

pub fn planet_motion(time: Res<Time>, mut boddies: Query<(&mut Transform, &OrbitalBody)>) {
    let delta = time.delta_seconds();
    for (mut tra, orb) in boddies.iter_mut() {
        tra.translation += delta * orb.velocity;
    }
}
