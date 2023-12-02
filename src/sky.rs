//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats

use bevy::render::render_resource::AsBindGroup;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError},
    },
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cubemap_materials: ResMut<Assets<CubemapMaterial>>,
) {
    commands.spawn(MaterialMeshBundle::<CubemapMaterial> {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
        material: cubemap_materials.add(CubemapMaterial { ..default() }),
        ..default()
    });
}

pub struct SkyBoxPlugin;

impl Plugin for SkyBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CubemapMaterial>::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (animate_sky,));
    }
}

pub fn animate_sky(time: Res<Time>, mut cubemap_materials: ResMut<Assets<CubemapMaterial>>) {
    for material in cubemap_materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct CubemapMaterial {
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub noise_texture: Option<Handle<Image>>,
    #[texture(3, dimension = "3d")]
    #[sampler(4)]
    pub volume_texture: Option<Handle<Image>>,
}

impl Material for CubemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/skybox.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

// impl AsBindGroup for CubemapMaterial {
//     type Data = Self;
//     // type Data = ();

//     fn as_bind_group(
//         &self,
//         layout: &BindGroupLayout,
//         render_device: &RenderDevice,
//         images: &RenderAssets<Image>,
//         _fallback_image: &FallbackImage,
//     ) -> Result<PreparedBindGroup<Self::Data>, AsBindGroupError> {
//         let base_color_texture = self
//             .base_color_texture
//             .as_ref()
//             .ok_or(AsBindGroupError::RetryNextUpdate)?;
//         let image = images
//             .get(base_color_texture)
//             .ok_or(AsBindGroupError::RetryNextUpdate)?;
//         let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
//             entries: &[
//                 BindGroupEntry {
//                     binding: 0,
//                     resource: BindingResource::TextureView(&image.texture_view),
//                 },
//                 BindGroupEntry {
//                     binding: 1,
//                     resource: BindingResource::Sampler(&image.sampler),
//                 },
//             ],
//             label: Some("cubemap_texture_material_bind_group"),
//             layout,
//         });

//         Ok(PreparedBindGroup {
//             bind_group,
//             bindings: vec![
//                 OwnedBindingResource::TextureView(image.texture_view.clone()),
//                 OwnedBindingResource::Sampler(image.sampler.clone()),
//             ],
//             data: Self {
//                 base_color_texture: None,
//             },
//         })
//     }

//     fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//             entries: &[
//                 // Cubemap Base Color Texture
//                 BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture {
//                         multisampled: false,
//                         sample_type: TextureSampleType::Float { filterable: true },
//                         view_dimension: TextureViewDimension::Cube,
//                     },
//                     count: None,
//                 },
//                 // Cubemap Base Color Texture Sampler
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::Filtering),
//                     count: None,
//                 },
//             ],
//             label: None,
//         })
//     }
// }
