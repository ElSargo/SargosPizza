use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, render::render_resource::ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct SunMaterial {
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub noise_texture: Option<Handle<Image>>,
    #[texture(3, dimension = "3d")]
    #[sampler(4)]
    pub volume_texture: Option<Handle<Image>>,
}

impl Material for SunMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sun.wgsl".into()
    }
}

pub struct SunPlugin;
impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SunMaterial>::default());
    }
}
