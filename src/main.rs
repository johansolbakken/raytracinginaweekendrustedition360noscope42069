use std::fs::File;
use std::io::Write as _;

use camera::Camera;
use ray::Ray;
use scene::Scene;

mod scene;
mod camera;
mod ray;

// --------------- Utils ---------------

fn vec3_to_u32(vec: &glm::DVec4) -> u32 {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    return r + (g << 8) + (b << 16);
}

// --------------- Utils ---------------


// --------------- Renderer ---------------

struct HitPayload {
    hit_distance: f64,
    world_position: glm::DVec3,
    world_normal: glm::DVec3,
    object_index: i32,
}

impl Default for HitPayload {
    fn default() -> Self {
        Self {
            hit_distance: Default::default(),
            world_position: glm::dvec3(0.0, 0.0, 0.0),
            world_normal: glm::dvec3(0.0, 0.0, 0.0),
            object_index: Default::default(),
        }
    }
}

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

    fn render(&mut self, camera: &Camera, scene: &Scene) {
        self.frame_index += 1;
        for j in 0..self.height {
            println!("Scanlines remaining: {}", self.height - 1 - j);
            for i in 0..self.width {
                let traced_color = self.per_pixel(i, j, camera, scene);
                self.accum[i + j * self.width] = self.accum[i + j * self.width] + traced_color;

                let color = self.accum[i + j * self.width] / self.frame_index as f64;
                self.pixels[i + j * self.width] = vec3_to_u32(&color);
            }
        }
    }

    fn per_pixel(&mut self, x: usize, y: usize, camera: &Camera, scene: &Scene) -> glm::DVec4 {
        let u = x as f64 / self.width as f64;
        let v = y as f64 / self.height as f64;
        let ray = camera.get_ray(u, v);

        let mut color = glm::dvec3(0.0, 0.0, 0.0);
        let mut multiplier = 1.0;

        let bounces = 5;
        for _ in 0..bounces {
            let payload: HitPayload = self.trace_ray(&ray, camera, scene);

            if payload.hit_distance < 0.0 {
                let unit_direction = glm::normalize(*ray.direction());
                let t = 0.5 * (unit_direction.y + 1.0);
                let sky_color =
                    glm::dvec3(1.0, 1.0, 1.0) * (1.0 - t) + glm::dvec3(0.5, 0.7, 1.0) * t;
                color = color + sky_color * multiplier;
            }

            color = color + glm::dvec3(1.0, 0.0, 0.0);

            multiplier *= 0.5;
        }

        return glm::dvec4(color.x, color.y, color.z, 1.0);
    }

    fn trace_ray(&mut self, ray: &Ray, camera: &Camera, scene: &Scene) -> HitPayload {
        let mut closest_sphere = -1;
        let mut hit_distance = f64::INFINITY;

        for (i, sphere) in scene.spheres.iter().enumerate() {
            // Sphere intersection
            let origin = *ray.origin() - *sphere.center();
            let a = glm::dot(*ray.direction(), *ray.direction());
            let b = 2.0 * glm::dot(origin, *ray.direction());
            let c = glm::dot(origin, origin) - sphere.radius() * sphere.radius();
            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                continue;
            }

            let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);
            if closest_t > 0.0 && closest_t < hit_distance {
                closest_sphere = i as i32;
                hit_distance = closest_t;
            }
        }

        if closest_sphere < 0 {
            self.miss(ray)
        } else {
            self.closest_hit(ray, hit_distance, closest_sphere, scene)
        }
    }

    fn closest_hit(
        &mut self,
        ray: &Ray,
        hit_distance: f64,
        object_index: i32,
        scene: &Scene,
    ) -> HitPayload {
        let sphere = &scene.spheres[object_index as usize];

        let origin = *ray.origin() - *sphere.center();
        let mut world_position = origin + *ray.direction() * hit_distance;
        let world_normal = glm::normalize(world_position);
        world_position = world_position + *sphere.center();

        HitPayload {
            hit_distance,
            world_position,
            world_normal,
            object_index,
        }
    }

    fn miss(&mut self, ray: &Ray) -> HitPayload {
        HitPayload {
            hit_distance: -1.0,
            ..Default::default()
        }
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
    let scene = Scene::new();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    for _ in 0..1 {
        renderer.render(&camera, &scene);
    }
    renderer.save("image.ppm");
}
