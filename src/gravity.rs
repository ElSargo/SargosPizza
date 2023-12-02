pub const G: f32 = 20.0;
use bevy::prelude::*;
#[derive(Component)]
pub struct GravityBody {
    pub mass: f32,
}

#[derive(Component)]
pub struct OrbitalBody {
    pub velocity: Vec3,
}

pub fn apply_gravity(
    time: Res<Time>,
    mut boddies: Query<(Entity, &mut OrbitalBody, &Transform)>,
    mut gravity_boddies: Query<(Entity, &GravityBody, &Transform)>,
) {
    let delta = time.delta_seconds();
    for (id, mut orbital, tra1) in boddies.iter_mut() {
        let mut force = Vec3::ZERO;
        for (id2, GravityBody { mass }, tra2) in gravity_boddies.iter_mut() {
            if id == id2 {
                continue;
            }
            let gravity = mass / tra1.translation.distance_squared(tra2.translation)
                * (tra2.translation - tra1.translation).normalize();
            force += gravity
        }
        orbital.velocity += delta * force * G;
    }
}

pub fn planet_motion(time: Res<Time>, mut boddies: Query<(&mut Transform, &OrbitalBody)>) {
    let delta = time.delta_seconds();
    for (mut tra, orb) in boddies.iter_mut() {
        tra.translation += delta * orb.velocity;
    }
}
