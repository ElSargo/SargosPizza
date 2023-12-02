//! Bevy has an optional prepass that is controlled per-material. A prepass is a rendering pass that runs before the main pass.
//! It will optionally generate various view textures. Currently it supports depth, normal, and motion vector textures.
//! The textures are not generated for any material using alpha blending.

use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

pub struct FogPlugin;
impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FogMaterial> {
            // This material only needs to read the prepass textures,
            // but the meshes using it should not contribute to the prepass render, so we can disable it.
            prepass_enabled: false,
            ..default()
        });
        app.insert_resource(Msaa::Off);
    }
}

/// set up a simple 3D scene
pub fn make_fog_quad(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut depth_materials: ResMut<Assets<FogMaterial>>,
) -> Entity {
    // camera

    // A quad that shows the outputs of the prepass
    // To make it easy, we just draw a big quad right in front of the camera.
    // For a real application, this isn't ideal.
    commands
        .spawn((
            MaterialMeshBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(1.0, 1.0)).into()),
                visibility: Visibility::Visible,
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -0.2,
                    },
                    ..default()
                },
                material: depth_materials.add(FogMaterial {
                    settings: ShowPrepassSettings::default(),
                }),
                ..default()
            },
            NotShadowCaster,
        ))
        .id()
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, Default, ShaderType)]
struct ShowPrepassSettings {
    thick: f32,
}

// This shader simply loads the prepass texture and outputs it directly
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FogMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    // This needs to be transparent in order to show the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
