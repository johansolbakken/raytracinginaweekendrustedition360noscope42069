pub struct Ray {
    origin: glm::DVec3,
    direction: glm::DVec3,
}

impl Ray {
    pub fn new(origin: glm::DVec3, direction: glm::DVec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &glm::DVec3 {
        &self.origin
    }

    pub fn direction(&self) -> &glm::DVec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> glm::DVec3 {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(glm::dvec3(0.0, 0.0, 0.0), glm::dvec3(0.0, 0.0, 0.0))
    }
}
