use std::fs::File;
use std::io::Write as _;

use crate::{
    camera::Camera,
    ray::Ray,
    scene::{Scene, Sphere},
    utils::{self, near_zero, random_f64, random_vec3_range, some_kind_of_gamma},
    vec3::Color3,
};

fn vec4_to_u32(vec: &glm::DVec4) -> u32 {
    let r = (255.0 * vec.x) as u32;
    let g = (255.0 * vec.y) as u32;
    let b = (255.0 * vec.z) as u32;
    let a = (255.0 * vec.w) as u32;

    return r + (g << 8) + (b << 16) + (a << 24);
}

// --------------- Utils ---------------

// --------------- Renderer ---------------

#[derive(Clone)]
struct HitPayload {
    hit_distance: f64,
    world_position: glm::DVec3,
    world_normal: glm::DVec3,
    object_index: i32,

    // ray tracing in one weekend
    front_face: bool,
}

impl Default for HitPayload {
    fn default() -> Self {
        Self {
            hit_distance: Default::default(),
            world_position: glm::dvec3(0.0, 0.0, 0.0),
            world_normal: glm::dvec3(0.0, 0.0, 0.0),
            object_index: Default::default(),
            front_face: Default::default(),
        }
    }
}

pub struct Renderer {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
    accum: Vec<glm::DVec4>,
    frame_index: usize,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            pixels: vec![],
            width: 0,
            height: 0,
            accum: vec![],
            frame_index: 0,
        }
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![0; width * height];
        self.accum = vec![glm::dvec4(0.0, 0.0, 0.0, 1.0); width * height];
        self.frame_index = 0;
        self.width = width;
        self.height = height;
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        self.frame_index += 1;
        for j in 0..self.height {
            for i in 0..self.width {
                let color = self.per_pixel(i, j, camera, scene);
                self.accum[i + j * self.width] = self.accum[i + j * self.width] + color;

                let mut accumulated_color = self.accum[i + j * self.width];
                accumulated_color = accumulated_color / self.frame_index as f64;
                accumulated_color = glm::clamp(
                    accumulated_color,
                    glm::dvec4(0.0, 0.0, 0.0, 0.0),
                    glm::dvec4(1.0, 1.0, 1.0, 1.0),
                );
                self.pixels[i + j * self.width] = vec4_to_u32(&accumulated_color);
            }
        }
    }

    fn per_pixel(&mut self, x: usize, y: usize, camera: &Camera, scene: &Scene) -> glm::DVec4 {
        let u = (x as f64 + random_f64()) / self.width as f64;
        let v = (y as f64 + random_f64()) / self.height as f64;
        let mut ray = camera.get_ray(u, v);

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
                break;
            }

            let light_dir = glm::normalize(glm::dvec3(-1.0, -1.0, -1.0));
            let light_intensity = glm::max(glm::dot(payload.world_normal, -light_dir), 0.0); // cos(angle)

            let sphere = &scene.spheres[payload.object_index as usize];
            let material = &scene.materials[sphere.material_index()];

            let mut sphere_color = material.albedo;
            sphere_color = sphere_color * light_intensity;
            color = color + sphere_color * multiplier;

            multiplier *= 0.5;

            let new_origin = payload.world_position + payload.world_normal * 0.0001;
            let new_direction = glm::reflect(
                *ray.direction(),
                payload.world_normal + random_vec3_range(-0.5, 0.5) * material.roughness,
            );
            ray = Ray::new(new_origin, new_direction);
        }

        return glm::dvec4(color.x, color.y, color.z, 1.0);
    }

    fn trace_ray(&mut self, ray: &Ray, _camera: &Camera, scene: &Scene) -> HitPayload {
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
            ..Default::default()
        }
    }

    fn miss(&mut self, _ray: &Ray) -> HitPayload {
        HitPayload {
            hit_distance: -1.0,
            ..Default::default()
        }
    }

    // ---------------------------- recursive render ----------------------------

    // This renderer matches more closely the ray tracing in weekend book
    // Instead of doing anti-aliasing by sampling the pixel, we just accumulate the color

    pub fn render_recurse(&mut self, camera: &Camera, scene: &Scene) {
        let max_depth = 50;

        self.frame_index += 1;

        for j in 0..self.height {
            for i in 0..self.width {
                // Calculating u, v
                let u = (i as f64 + random_f64()) / (self.width - 1) as f64;
                let v = (j as f64 + random_f64()) / (self.height - 1) as f64;

                // Calculating ray
                let ray = camera.get_ray(u, v);
                let color = self.pixel_color(&ray, scene, max_depth);

                // Accumulating color
                self.accum[i + j * self.width] =
                    self.accum[i + j * self.width] + glm::dvec4(color.x, color.y, color.z, 1.0);

                // Averaging color
                let mut accum_color = self.accum[i + j * self.width] / self.frame_index as f64;
                accum_color = glm::clamp(
                    accum_color,
                    glm::dvec4(0.0, 0.0, 0.0, 0.0),
                    glm::dvec4(1.0, 1.0, 1.0, 1.0),
                );
                accum_color = some_kind_of_gamma(&accum_color);

                // Setting pixel color
                self.pixels[i + j * self.width] = vec4_to_u32(&accum_color);
            }
        }
    }

    fn pixel_color(&mut self, ray: &Ray, scene: &Scene, depth: u32) -> Color3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return glm::dvec3(0.0, 0.0, 0.0);
        }

        let mut rec = HitPayload::default();
        if self.world_hit(scene, ray, 0.001, f64::MAX, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = glm::dvec3(0.0, 0.0, 0.0);

            let sphere = &scene.spheres[rec.object_index as usize];
            if self.scatter(
                sphere.material_index(),
                scene,
                ray,
                &mut rec,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * self.pixel_color(&scattered, scene, depth - 1);
            }

            return glm::dvec3(0.0, 0.0, 0.0);
        }

        // Background gradient
        let unit_direction = glm::normalize(*ray.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        return glm::dvec3(1.0, 1.0, 1.0) * (1.0 - t) + glm::dvec3(0.5, 0.7, 1.0) * t;
    }

    fn world_hit(
        &mut self,
        scene: &Scene,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitPayload,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for (i, sphere) in scene.spheres.iter().enumerate() {
            let mut temp_rec = HitPayload::default();
            if self.sphere_hit(sphere, scene, ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.hit_distance;
                *rec = temp_rec.clone();
                rec.object_index = i as i32;
            }
        }

        return hit_anything;
    }

    fn sphere_hit(
        &mut self,
        sphere: &Sphere,
        _scene: &Scene,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitPayload,
    ) -> bool {
        let oc = *r.origin() - *sphere.center();
        let a = glm::length(*r.direction()).powf(2.0);
        let half_b = glm::dot(oc, *r.direction());
        let c = glm::length(oc).powf(2.0) - sphere.radius() * sphere.radius();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.hit_distance = root;
        rec.world_position = r.at(rec.hit_distance);
        let outward_normal = (rec.world_position - *sphere.center()) / sphere.radius();
        let front_face = glm::dot(*r.direction(), outward_normal) < 0.0;
        rec.world_normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        // Offset the hit point to avoid shadow acne
        rec.world_position = rec.world_position + rec.world_normal * 0.0001;
        rec.front_face = front_face;

        return true;
    }

    fn scatter(
        &mut self,
        material_index: usize,
        scene: &Scene,
        r_in: &Ray,
        rec: &mut HitPayload,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let material = &scene.materials[material_index];

        // Dielectrics
        if material.glass {
            *attenuation = glm::dvec3(1.0, 1.0, 1.0);
            let refraction_ratio = if rec.front_face {
                1.0 / material.refraction_index
            } else {
                material.refraction_index
            };

            // Why do I not get hollow sphere as in the book?

            let unit_direction = glm::normalize(*r_in.direction());
            let cos_theta = glm::min(glm::dot(-unit_direction, rec.world_normal), 1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction = if cannot_refract
                || (utils::reflectance(cos_theta, refraction_ratio) > utils::random_f64())
            {
                glm::reflect(unit_direction, rec.world_normal)
            } else {
                glm::refract(unit_direction, rec.world_normal, refraction_ratio)
            };

            *scattered = Ray::new(rec.world_position, direction);
            return true;
        }

        // Lamberian and metal
        let mut scatter_direction = glm::reflect(
            glm::normalize(*r_in.direction()),
            rec.world_normal + random_vec3_range(-0.5, 0.5) * material.roughness,
        );

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = rec.world_normal;
        }

        *scattered = Ray::new(rec.world_position, scatter_direction);
        *attenuation = material.albedo;

        return glm::dot(*scattered.direction(), rec.world_normal) > 0.0;
    }

    // ---------------------------- recursive render ----------------------------

    pub fn save(&self, filename: &str) {
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
