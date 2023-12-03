mod fog;
mod gravity;
mod orbit_cam;
mod planet;
mod sky;
mod sun;
use std::f32::consts::TAU;

use bevy::{math::vec3, prelude::*};
use gravity::{apply_gravity, planet_motion, G};
use planet::{get_mat, PlanetMaterial, PlanetReasources};
use rand::prelude::*;
use sun::SunMaterial;

use crate::gravity::{Mass, OrbitalBody};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..Default::default()
            }),
            orbit_cam::OrbitCamPlugin,
            sky::SkyBoxPlugin,
            sun::SunPlugin,
            planet::PlanetPlugin,
            fog::FogPlugin, // TemporalAntiAliasPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, apply_gravity)
        .add_systems(Update, planet_motion)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sun_mats: ResMut<Assets<SunMaterial>>,
    mut planet_mats: ResMut<Assets<PlanetMaterial>>,
    mut plr: ResMut<PlanetReasources>,
) {
    let sun_rad = 25.0;
    let star_material = sun_mats.add(SunMaterial::default());
    {
        let planet_mesh = meshes.add(
            shape::UVSphere {
                radius: 1.0,
                sectors: 30,
                stacks: 30,
            }
            .into(),
        );
        // let pm = mats.add(StandardMaterial { ..default() });
        let mut rng = thread_rng();
        for _ in 0..200 {
            let rad: f32 = rng.gen_range(0.2..1.0_f32).powi(2) * 20.;
            let mass = 10.0 * rad * rad;
            let x = rng.gen_range(0.0..TAU);
            let y = rng.gen_range(0.0..TAU);
            let z = rng.gen_range(0.0..TAU);
            let mut trans = Transform::from_xyz(rng.gen_range(200.0..1000.0), 0.0, 0.0);
            trans.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XYZ, x, y, z));
            trans.scale *= rad;
            let dist = trans.translation.length();

            // let axis = trans.translation.normalize();
            // let tangent = axis.cross(Vec3::Y);
            // let rot = Quat::from_axis_angle(axis, rng.gen_range(0.0..TAU));
            // let mut vel = rot.mul_vec3(tangent);

            let mut vel = spin_velocity(trans);
            vel.y = rng.gen_range(-1.0..1.0);
            // vel.y /= 10.0;
            vel *= 100000.0 * sun_rad * sun_rad * G / (dist * dist) / mass;

            commands.spawn((
                MaterialMeshBundle {
                    mesh: planet_mesh.clone(),
                    material: get_mat(dist, rad, sun_rad, &mut plr, &mut planet_mats),
                    visibility: Visibility::Visible,
                    transform: trans,
                    ..default()
                },
                OrbitalBody { velocity: vel },
                Mass { mass, radius: rad },
            ));
        }
    }
    {
        let star_mesh = meshes.add(
            shape::UVSphere {
                radius: sun_rad,
                sectors: 30,
                stacks: 30,
            }
            .into(),
        );
        commands.spawn((
            MaterialMeshBundle {
                mesh: star_mesh,
                visibility: Visibility::Visible,
                material: star_material,
                ..default()
            },
            Mass {
                mass: 1000.0 * sun_rad * sun_rad,
                radius: sun_rad,
            },
        ));
    }
}

fn spin_velocity(trans: Transform) -> Vec3 {
    let axis = trans.translation.xz().normalize();
    let ang = axis.x.atan2(axis.y) + std::f32::consts::FRAC_PI_2;
    vec3((ang).sin(), 0.0, (ang).cos())
}
