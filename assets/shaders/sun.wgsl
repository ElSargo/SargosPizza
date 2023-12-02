#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
struct SunMaterial {
    time: f32,
};


@group(1) @binding(0)
var<uniform> material: SunMaterial;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4(500.0,400.8,300.0,1.0);

}
