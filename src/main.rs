mod fog;
mod gravity;
mod orbit_cam;
mod planet;
mod sky;
mod sun;
use std::f32::consts::TAU;

use bevy::prelude::*;
use gravity::{apply_gravity, planet_motion, G};
use planet::{get_mat, PlanetMaterial, PlanetReasources};
use rand::prelude::*;
use sun::SunMaterial;

use crate::gravity::{GravityBody, OrbitalBody};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            orbit_cam::OrbitCamPlugin,
            sky::SkyBoxPlugin,
            sun::SunPlugin,
            planet::PlanetPlugin,
            fog::FogPlugin, // TemporalAntiAliasPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_gravity, planet_motion))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sun_mats: ResMut<Assets<SunMaterial>>,
    mut planet_mats: ResMut<Assets<PlanetMaterial>>,
    mut plr: ResMut<PlanetReasources>,
    mut asset_server: ResMut<AssetServer>,
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
        for _ in 0..1000 {
            let rad: f32 = rng.gen_range(-20.0..0.0_f32).exp() * 20.;
            // let rad = 10.;
            let mass = 10.0 * rad * rad;
            let x = rng.gen_range(0.0..TAU);
            let y = rng.gen_range(0.0..TAU);
            let z = rng.gen_range(0.0..TAU);
            let mut trans = Transform::from_xyz(rng.gen_range(250.0..1000.0), 0.0, 0.0);
            trans.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XYZ, x, y, z));
            trans.translation.y /= 10.0;
            trans.scale *= rad;
            let dist = trans.translation.length();
            let axis = trans.translation.normalize();
            let tangent = axis.cross(Vec3::Y);
            let rot = Quat::from_axis_angle(axis, rng.gen_range(0.0..TAU));
            let mut vel = rot * tangent;
            vel.y /= 10.0;
            vel = vel.normalize() * 100.0 * sun_rad * sun_rad * G * 5. / (dist * dist);

            commands.spawn((
                MaterialMeshBundle {
                    mesh: planet_mesh.clone(),
                    material: get_mat(
                        dist,
                        rad,
                        sun_rad,
                        &mut plr,
                        &mut planet_mats,
                        &mut asset_server,
                    ),
                    visibility: Visibility::Visible,
                    transform: trans,
                    ..default()
                },
                OrbitalBody { velocity: vel },
                GravityBody { mass },
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
            GravityBody {
                mass: 100.0 * sun_rad * sun_rad,
            },
        ));
    }
}
