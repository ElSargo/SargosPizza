struct CubemapMaterial {
    time: f32,
};


@group(2) @binding(0)
var<uniform> material: CubemapMaterial;

@group(2) @binding(1) var skybox: texture_cube<f32>;
@group(2) @binding(2) var skybox_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings


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

fn rayleigh(costh: f32) -> f32 {
    return 3.0 / (16.0 * 3.14159265358979323846) * (1.0 + costh * costh);
}

fn fre(cos_theta_incident: f32) -> f32 {
    let p = 1.0 - cos_theta_incident;
    let p2 = p * p;
    return p2 * p2 * p;
}

fn fnexp(x: f32) -> f32 {
    let a = 0.2 * x + 1.;
    let b = a * a;
    return b * b;
}

fn fnexp3(x: vec3<f32>) -> vec3<f32> {
    let a = 0.2 * min(x, vec3(6.)) + 1.;
    let b = a * a;
    return b * b;
}

fn iSph(  ro: vec3<f32>,  rd: vec3<f32>,  ce: vec3<f32>, ra: f32 ) -> vec2<f32> {
    let oc = ro - ce;
    let b = dot( oc, rd );
    let c = dot( oc, oc ) - ra*ra;
    var h = b*b - c;
    if h < 0.0 { return vec2(-1.0); }// no intersection
    h = sqrt( h );
    return vec2( -b-h, -b+h );
}


fn iElips1( ro: vec3<f32>, rd: vec3<f32>, ce: vec3<f32>, rad: vec3<f32>) -> f32 {
    let oc = ro - ce;
    
    let ocn = oc / rad;
    let rdn = rd / rad;
    
    let a = dot( rdn, rdn );
	let b = dot( ocn, rdn );
	let c = dot( ocn, ocn );
	let h = b*b - a*(c - 1.0);
	if h<0.0 { return -1.0; }
	return (-b - sqrt( h ))/a;
}

fn iElips( ro: vec3<f32>, rd: vec3<f32>, ce: vec3<f32>, rad: vec3<f32>) -> vec2<f32> {
    let a = iElips1(  ro,  rd,  ce, rad);
    if a < -0.5 { return vec2(-1.0);}
    let ro2 = ro + rd*100.;
    let b = iElips1( ro2, -rd,  ce, rad);
    return vec2(a,100.0 - b);
}

fn rot(a: f32) -> mat2x2<f32> {
    let s = sin(a);
    let c = cos(a);
    return mat2x2(c,-s,s,c);
}

fn erot(p : vec3<f32>, ax: vec3<f32>,  ro: f32) -> vec3<f32> {
    return mix(dot(p,ax)*ax,p,cos(ro))+sin(ro)*cross(ax,p);
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let ro = mesh_view_bindings::view.world_position.xyz;
    var rd = normalize(mesh.world_position.xyz - ro );

    let base = vec3(0.0);
    let lines = 10.;
    let hole_dir = normalize(vec3(1.0,-0.1,0.4));
    let hole_ce = hole_dir * 10.;
    let bhi = iSph(  vec3(0.0),  rd,  hole_ce, 2.0);
    let disk_rad = vec3(3.,0.1,3.0);
    var smoke = vec3(0.0) ;
    var dens = 0.;
    let blak = dot(rd,hole_dir);
    let rt = 10.*exp(-20.0*(-2.0*blak+2.0));
    rd = erot(rd,hole_dir, rt);
    var tex = textureSample(skybox, skybox_sampler, rd).rgb;
    tex *= 1.0 - smoothstep(0.98,0.981,blak);
    return vec4(4.*tex,1.0);

    // var eli = iElips(  vec3(0.0),  rd,  hole_ce , disk_rad );
    // var end = 0.0;
    // if bhi.x > -0.5 {
    //     end = min(bhi.x, eli.y);
    // } else {
    //     end = eli.y;
    // }
    // for ( var T = eli.x; T<end ; T += 0.1) {
    //     let p = rd*T;
    //     let dp = p - hole_ce;
    //     let r = dp.xz * rot(material.time* 0.1);
    //     let sp = hole_ce + vec3(r.x,dp.y,r.y);
    //     let d = max(0.,sin(sp.x * 20. )*sin(p.y * 20.)*sin(sp.z * 20. ) ) ;
    //     smoke += exp(-dens)*d;
    //     dens += d*10.;
    // }

    // if bhi.x > -0.5  && blak < 0.99{
    //     let pos = bhi.x * rd;
    //     let nor = normalize( pos - hole_ce);
    //     let rrd = refract(rd,nor,0.42);
    //     eli = iElips( pos,  rrd,  hole_ce , disk_rad);
    //     let sph2 = iSph(  pos,  rrd,  hole_ce, 2.0);
    //     for ( var T = max(sph2.y,eli.x); T<eli.y; T += 0.1) {
    //         let p = pos + rrd*T;
    //     let dp = p - hole_ce;
    //     let r = dp.xz * rot(material.time * 0.1);
    //     let sp = hole_ce + vec3(r.x,dp.y,r.y);
    //     let d = max(0.,sin(sp.x * 20.)*sin(p.y * 20.)*sin(sp.z * 20. ) ) ;
    //     smoke += exp(-dens)*d;
    //     dens += d*10.;
    //     }
    // } else {
    //     // return vec4(rd,1.);
    // }

    // return vec4(, 1.0);

    // let g = refract()
    
    // var sig = vec3(1.0)*pow(0.5+0.5*sin(5000.*blak),4.0);
    // let band_signal = 1. - max(0.,eli.y - eli.x);
    // sig *= 1.0 - smoothstep(0.98,0.981,min(blak,band_signal));
    // let col = mix(base,sig,smoothstep(0.972,0.975,blak));
    
    // // return vec4(col*100., 0.);
    // return vec4(smoke);
    // return vec4(1.-exp(- max(0.,eli.y - eli.x)));


}
