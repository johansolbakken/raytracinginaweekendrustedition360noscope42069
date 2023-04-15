use crate::vec3::vec3;

pub struct Material {
    pub albedo: vec3,
    pub roughness: f64,
    pub metallic: f64,
}

pub struct Sphere {
    center: glm::DVec3,
    radius: f64,
    material_index: usize,
}

impl Sphere {
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
                },
                Material {
                    albedo: glm::dvec3(0.2, 0.3, 1.0),
                    roughness: 0.1,
                    metallic: 0.0,
                },
            ],
        }
    }
}
