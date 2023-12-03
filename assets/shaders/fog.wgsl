#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_view_bindings as view_bindings,
    prepass_utils,
    forward_io::VertexOutput,
}
#import bevy_render::{
    view::View
}

struct FogSettings {
    thick: f32
}
@group(2) @binding(0) var<uniform> settings: FogSettings;


fn fog_int(roa: vec3<f32>, rda: vec3<f32>, t: f32) -> f32 {
    let rd = rda * vec3(1.0,2.0,1.0);
    let ro = roa * vec3(1.0,2.0,1.0);
    let a = (rd.y * ro.z - ro.y * rd.z);
    let b = (rd.y * ro.z - ro.y * rd.z);
    let c = ro.x * rd.x + ro.y * rd.y + ro.z * rd.z + rd.x * rd.x * t + rd.y * rd.y * t + rd.z * rd.z * t ;
    let d = rd.x * rd.x * ( ro.y * ro.y + ro.z * ro.z ) + a * a - 2.0 * ro.x * rd.x * ( ro.y * rd.y + ro.z * rd.z ) + ro.x * ro.x * ( rd.y * rd.y + rd.z * rd.z ) ;
    let e =  rd.x * rd.x * ( ro.y * ro.y + ro.z * ro.z ) + b * b - 2.0 * ro.x * rd.x * ( ro.y * rd.y + ro.z * rd.z ) + ro.x * ro.x * ( rd.y * rd.y + rd.z * rd.z ) ;
    return atan( c / sqrt(d) ) / sqrt(e);
}


@fragment
fn fragment(
#ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
#endif
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
#ifndef MULTISAMPLED
    let sample_index = 0u;
#endif
    let ro = view_bindings::view.world_position.xyz;
    let rd = normalize(mesh.world_position.xyz - ro );
    let depth = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
    let depth_lin = 1./depth;
    let fog = fog_int(ro, rd, depth_lin);
    return vec4(2.0,0.5,1.25, 0.95 - 0.95*exp(-fog*100.));

}
