#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::{
    mesh_view_bindings as view_bindings,
}
// we can import items from shader modules in the assets folder with a quoted path
struct PlanetMaterial {
    time: f32,
    sun_rad: f32,
    rad: f32,
    pen: vec3<f32>,
};


@group(2) @binding(0)
var<uniform> material: PlanetMaterial;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;


fn scatter(costheta: f32) -> f32 {
    let ct = clamp(-1.0,1.0,costheta);
    let cm1 = ct - 1.0;
    // return (ct + 0.5) * cm1 * cm1;
    return exp(min(0.0,4.*ct));
    
}

fn mie(costh: f32) -> f32 {
    // This function was optimized to minimize (delta*delta)/reference in order to capture
    // the low intensity behavior.
    let params = array(
        9.805233e-06,
        -6.500000e+01,
        -5.500000e+01,
        8.194068e-01,
        1.388198e-01,
        -8.370334e+01,
        7.810083e+00,
        2.054747e-03,
        2.600563e-02,
        -4.552125e-12
    );

    let p1 = costh + params[3];
    let expValues: vec4<f32> = exp(vec4(params[1] * costh + params[2], params[5] * p1 * p1, params[6] * costh, params[9] * costh));
    let expValWeight: vec4<f32> = vec4(params[0], params[4], params[7], params[8]);
    return dot(expValues, expValWeight) * 0.25;
}


fn fresnel(view: vec3<f32>,  normal: vec3<f32>,amount: f32) -> f32 {
	return pow((1.0 - clamp(dot(normalize(normal), normalize(view)), 0.0, 1.0 )), amount);
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let ro = view_bindings::view.world_position.xyz;
    let rd = normalize(mesh.world_position.xyz - ro );

    let pos = mesh.world_position.xyz;
    let nor = mesh.world_normal.xyz;
    let d2 = dot(pos,pos);
    let d = sqrt(d2);
    let sun_brightness = material.sun_rad * material.sun_rad * 5000.0 / d2 ;
    let sun_color = vec3(1.,0.8,0.4);
    let to_sun = normalize(pos);
    let nor_dot_sun = dot(-nor,to_sun);
    let scattered_light = 
    scatter(nor_dot_sun )*
    material.pen * mie(dot(rd,-to_sun)) * 10.;
    let refl = reflect(rd,nor);
    let fre = fresnel(-rd,nor,1.0);
    let spec = pow(0.5+0.5*dot(refl,-to_sun), 10.0);
    let col = textureSample(material_color_texture,material_color_sampler, mesh.uv).rgb * 
    (scattered_light + (.2 + fre) * vec3(1.,1.2,2.0) * 0.2 + sun_color *  sun_brightness * ( max(0., nor_dot_sun ) + spec * fre )) ;
    return vec4(col,1.0);
    // return vec4(fre*spec*10.);

}
