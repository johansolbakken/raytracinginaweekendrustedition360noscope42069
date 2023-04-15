use std::fs::File;
use std::io::Write as _;

// --------------- Utils ---------------

fn vec3_to_u32(vec: &glm::DVec4) -> u32 {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    return r + (g << 8) + (b << 16);
}

// --------------- Utils ---------------

// --------------- Ray ---------------

struct Ray {
    origin: glm::DVec3,
    direction: glm::DVec3,
}

impl Ray {
    fn new(origin: glm::DVec3, direction: glm::DVec3) -> Ray {
        Ray { origin, direction }
    }

    fn origin(&self) -> &glm::DVec3 {
        &self.origin
    }

    fn direction(&self) -> &glm::DVec3 {
        &self.direction
    }

    fn at(&self, t: f64) -> glm::DVec3 {
        self.origin + self.direction * t
    }
}

// --------------- Ray ---------------

// --------------- Camera ---------------

struct Camera {
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
    fn new() -> Camera {
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

    fn on_resize(&mut self, width: usize, height: usize) {
        self.ray_directions = vec![glm::dvec3(0.0, 0.0, 0.0); width * height];

        let aspect_ratio = width as f64 / height as f64;
        self.viewport_width = aspect_ratio * self.viewport_height;
        self.horizontal = glm::dvec3(self.viewport_width, 0.0, 0.0);
        self.vertical = glm::dvec3(0.0, self.viewport_height, 0.0);

        self.on_update();
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.origin = glm::dvec3(x, y, 0.0);
        self.on_update();
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

// --------------- Camera ---------------

// --------------- Renderer ---------------

struct Renderer {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
    accum: Vec<glm::DVec4>,
    frame_index: usize,
}

impl Renderer {
    fn new() -> Renderer {
        Renderer {
            pixels: vec![],
            width: 0,
            height: 0,
            accum: vec![],
            frame_index: 0,
        }
    }

    fn on_resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![0; width * height];
        self.accum = vec![glm::dvec4(0.0, 0.0, 0.0, 1.0); width * height];
        self.frame_index = 0;
        self.width = width;
        self.height = height;
    }

    fn render(&mut self, camera: &Camera) {
        self.frame_index += 1;
        for j in 0..self.height {
            println!("Scanlines remaining: {}", self.height - 1 - j);
            for i in 0..self.width {
                let traced_color = self.per_pixel(i, j, camera);
                self.accum[i + j * self.width] = self.accum[i + j * self.width] + traced_color;

                let color = self.accum[i + j * self.width] / self.frame_index as f64;
                self.pixels[i + j * self.width] = vec3_to_u32(&color);
            }
        }
    }

    fn per_pixel(&mut self, x: usize, y: usize, camera: &Camera) -> glm::DVec4 {
        let u = x as f64 / self.width as f64;
        let v = y as f64 / self.height as f64;
        let ray = camera.get_ray(u, v);

        let unit_direction = glm::normalize(ray.direction().clone());
        let t = 0.5 * (unit_direction.y + 1.0);
        return glm::dvec4(1.0, 1.0, 1.0, 1.0) * (1.0 - t) + glm::dvec4(0.5, 0.7, 1.0, 1.0) * t;
    }

    fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes()).unwrap();

        for j in 0..self.height {
            let y = self.height - 1 - j;
            for i in 0..self.width {
                let x = i;
                let pixel = self.pixels[x + y * self.width];
                let r = (pixel >> 0) & 0xFF;
                let g = (pixel >> 8) & 0xFF;
                let b = (pixel >> 16) & 0xFF;
                let line = format!("{} {} {}\n", r, g, b);
                file.write_all(line.as_bytes()).unwrap();
            }
        }

        println!("Saved image to {}", filename);
    }
}

// --------------- Renderer ---------------

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    for _ in 0..1 {
        renderer.render(&camera);
    }
    renderer.save("image.ppm");
}
