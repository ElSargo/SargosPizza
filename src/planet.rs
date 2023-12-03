use bevy::math::vec3;
use bevy::render::render_resource::AsBindGroup;
use bevy::utils::hashbrown::HashMap;

use bevy::{prelude::*, render::render_resource::ShaderRef};
use rand::{random, thread_rng, Rng};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub sun_rad: f32,
    #[uniform(0)]
    pub rad: f32,
    #[uniform(0)]
    pub pen: Vec3,

    #[texture(1)]
    #[sampler(2)]
    pub color: Option<Handle<Image>>,
}

impl Material for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PlanetMaterial>::default());
        app.init_resource::<PlanetReasources>();
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum PlanetType {
    Terestrial,
    GasGiant,
    Habitable,
    Inhospitible,
}

#[derive(Resource)]
pub struct PlanetReasources(
    HashMap<PlanetType, Vec<(Handle<Image>, Option<Handle<PlanetMaterial>>)>>,
);

impl FromWorld for PlanetReasources {
    fn from_world(world: &mut World) -> Self {
        Self(load_planet_textures(
            &mut *world.get_resource_mut().unwrap(),
        ))
    }
}

fn load_planet_textures(
    asset_server: &mut AssetServer,
) -> HashMap<PlanetType, Vec<(Handle<Image>, Option<Handle<PlanetMaterial>>)>> {
    let mut textures = HashMap::new();
    for (pt, path) in [
        (PlanetType::Inhospitible, "textures/Alpine.png"),
        (PlanetType::GasGiant, "textures/Gaseous1.png"),
        (PlanetType::GasGiant, "textures/Gaseous2.png"),
        (PlanetType::GasGiant, "textures/Gaseous3.png"),
        (PlanetType::GasGiant, "textures/Gaseous4.png"),
        (PlanetType::Inhospitible, "textures/Icy.png"),
        (PlanetType::Inhospitible, "textures/Martian.png"),
        (PlanetType::Habitable, "textures/Savannah.png"),
        (PlanetType::Habitable, "textures/Swamp.png"),
        (PlanetType::Terestrial, "textures/Terrestrial1.png"),
        (PlanetType::Terestrial, "textures/Terrestrial2.png"),
        (PlanetType::Terestrial, "textures/Terrestrial3.png"),
        (PlanetType::Terestrial, "textures/Terrestrial4.png"),
        (PlanetType::Habitable, "textures/Tropical.png"),
        (PlanetType::Inhospitible, "textures/Venusian.png"),
        (PlanetType::Inhospitible, "textures/Volcanic.png"),
    ] {
        let handle = asset_server.load(path);
        textures.entry(pt).or_insert(vec![]).push((handle, None));
    }
    textures
}

fn get_rand_type(dist: f32, rad: f32) -> PlanetType {
    if rad > 7.0 && dist > 500.0 {
        PlanetType::GasGiant
    } else if dist < 350. {
        PlanetType::Inhospitible
    } else {
        if random() {
            PlanetType::Terestrial
        } else {
            PlanetType::Habitable
        }
    }
}

pub fn get_mat(
    dist: f32,
    rad: f32,
    sun_rad: f32,
    plr: &mut PlanetReasources,
    mats: &mut Assets<PlanetMaterial>,
) -> Handle<PlanetMaterial> {
    let pt = get_rand_type(dist, rad);
    let avalible = plr.0.get_mut(&pt).unwrap();
    let mut rng = thread_rng();
    let i = rng.gen_range(0..avalible.len());
    let (pic, mat) = &avalible[i];
    if let Some(mat) = mat {
        mat.clone()
    } else {
        let mat = mats.add(PlanetMaterial {
            sun_rad,
            color: Some(pic.clone()),
            pen: if pt == PlanetType::GasGiant {
                vec3(1.0, 0.5, 0.25)
            } else {
                vec3(0.0, 0.0, 0.0)
            },
            ..default()
        });

        avalible[i].1 = Some(mat.clone());
        mat
    }
}
