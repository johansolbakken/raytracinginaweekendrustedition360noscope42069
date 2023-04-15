use crate::ray::Ray;


pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,

    origin: glm::DVec3,
    horizontal: glm::DVec3,
    vertical: glm::DVec3,
    lower_left_corner: glm::DVec3,

    ray_directions: Vec<glm::DVec3>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            viewport_width: 2.0,
            viewport_height: 2.0,
            focal_length: 1.0,

            origin: glm::dvec3(0.0, 0.0, 0.0),
            horizontal: glm::dvec3(0.0, 0.0, 0.0),
            vertical: glm::dvec3(0.0, 0.0, 0.0),
            lower_left_corner: glm::dvec3(0.0, 0.0, 0.0),

            ray_directions: vec![],
        }
    }

    fn on_update(&mut self) {
        self.lower_left_corner = self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - glm::dvec3(0.0, 0.0, self.focal_length);
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        self.ray_directions = vec![glm::dvec3(0.0, 0.0, 0.0); width * height];

        let aspect_ratio = width as f64 / height as f64;
        self.viewport_height = 2.0;
        self.viewport_width = aspect_ratio * self.viewport_height;
        self.horizontal = glm::dvec3(self.viewport_width, 0.0, 0.0);
        self.vertical = glm::dvec3(0.0, self.viewport_height, 0.0);

        self.on_update();
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.origin = glm::dvec3(x, y, 0.0);
        self.on_update();
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
