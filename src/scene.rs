pub struct Sphere {
    center: glm::DVec3,
    radius: f64,
}

impl Sphere {
    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> &glm::DVec3 {
        &self.center
    }
}

pub struct Scene {
    pub(crate) spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![
                Sphere {
                    center: glm::dvec3(0.0, 0.0, -1.0),
                    radius: 0.5,
                },
                Sphere {
                    center: glm::dvec3(0.0, -100.5, -1.0),
                    radius: 100.0,
                },
            ],
        }
    }
}
