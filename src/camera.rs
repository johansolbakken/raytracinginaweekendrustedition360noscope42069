use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
    _v_fov: f64,

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
            _v_fov: 90.0,

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

    fn _move_to(&mut self, x: f64, y: f64) {
        self.origin = glm::dvec3(x, y, 0.0);
        self.on_update();
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }

    pub fn setup(
        &mut self,
        look_from: &Vec3,
        look_at: &Vec3,
        v_up: &Vec3,
        vfov: f64,
        _aperture: f64,
        focus_dist: f64,
    ) {
        self.origin = *look_from;

        let theta = glm::radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let aspect_ratio = self.viewport_width as f64 / self.viewport_height as f64;
        let viewport_width = aspect_ratio * viewport_height;

        let w = glm::normalize(*look_from - *look_at);
        let u = glm::normalize(glm::cross(*v_up, w));
        let v = glm::cross(w, u);

        self.origin = *look_from;
        self.horizontal = u * viewport_width * focus_dist;
        self.vertical = v * viewport_height * focus_dist;
        self.lower_left_corner =
            self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - w * focus_dist;
        // self.on_update();
    }
}
