use crate::{
    utils::{random_color, random_f64, random_f64_range},
    vec3::Vec3,
};

pub struct Material {
    pub albedo: Vec3,
    pub roughness: f64,
    pub metallic: f64,

    // ray tracing in one weekend
    pub glass: bool,
    pub refraction_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            roughness: 1.0,
            metallic: 0.0,
            glass: false,
            refraction_index: 1.0,
        }
    }
}

pub struct Sphere {
    center: glm::DVec3,
    radius: f64,
    material_index: usize,
}

impl Sphere {
    pub fn new(center: glm::DVec3, radius: f64, material_index: usize) -> Sphere {
        Sphere {
            center,
            radius,
            material_index,
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> &glm::DVec3 {
        &self.center
    }

    pub fn material_index(&self) -> usize {
        self.material_index
    }
}

pub struct Scene {
    pub(crate) spheres: Vec<Sphere>,
    pub(crate) materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
                Sphere {
                    center: glm::dvec3(0.0, 0.0, -2.0),
                    radius: 1.0,
                    material_index: 0,
                },
                Sphere {
                    center: glm::dvec3(0.0, -101.0, -2.0),
                    radius: 100.0,
                    material_index: 1,
                },
            ],
            materials: vec![
                Material {
                    albedo: glm::dvec3(1.0, 0.0, 1.0),
                    roughness: 0.0,
                    metallic: 0.0,
                    ..Default::default()
                },
                Material {
                    albedo: glm::dvec3(0.2, 0.3, 1.0),
                    roughness: 0.1,
                    metallic: 0.0,
                    ..Default::default()
                },
            ],
        }
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }
}

pub fn hittable_scene() -> Scene {
    let mut world = Scene::new();

    let ground_material = world.add_material(Material {
        albedo: glm::dvec3(0.5, 0.5, 0.5),
        roughness: 1.0,
        ..Default::default()
    });
    world.spheres.push(Sphere::new(
        glm::dvec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = glm::dvec3(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if glm::length(center - glm::dvec3(4.0, 0.2, 0.0)) > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color() * random_color();
                    world.add_material(Material {
                        albedo: albedo,
                        roughness: 1.0,
                        ..Default::default()
                    })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color();
                    let fuzz = random_f64_range(0.0, 0.5);
                    world.add_material(Material {
                        albedo: albedo,
                        roughness: fuzz,
                        ..Default::default()
                    })
                } else {
                    // glass
                    world.add_material(Material {
                        glass: true,
                        refraction_index: 1.5,
                        ..Default::default()
                    })
                };

                world
                    .spheres
                    .push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = world.add_material(Material {
        glass: true,
        refraction_index: 1.5,
        ..Default::default()
    });
    world
        .spheres
        .push(Sphere::new(glm::dvec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = world.add_material(Material {
        albedo: glm::dvec3(0.4, 0.2, 0.1),
        roughness: 1.0,
        ..Default::default()
    });
    world
        .spheres
        .push(Sphere::new(glm::dvec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = world.add_material(Material {
        albedo: glm::dvec3(0.7, 0.6, 0.5),
        roughness: 0.0,
        ..Default::default()
    });
    world
        .spheres
        .push(Sphere::new(glm::dvec3(4.0, 1.0, 0.0), 1.0, material3));

    world
}
