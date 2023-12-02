#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
struct PlanetMaterial {
    time: f32,
    sun_rad: f32,
    rad: f32,
};


@group(2) @binding(0)
var<uniform> material: PlanetMaterial;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let pos = mesh.world_position.xyz;
    let nor = mesh.world_normal.xyz;
    let d2 = dot(pos,pos);
    let d = sqrt(d2);
    let sun_brightness = material.sun_rad * material.sun_rad * 1000.0;
    let col = textureSample(material_color_texture,material_color_sampler, mesh.uv).rgb * (0.1 + vec3(1.,0.8,0.4) *  sun_brightness / d2 * max(0., dot(-nor,normalize(pos)))) ;
    return vec4(col,1.0);
    // return vec4(1.);

}
