use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    minimum: Vec3,
    maximum: Vec3,
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn minimum(&self) -> Vec3 {
        self.minimum
    }

    pub fn maximum(&self) -> Vec3 {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.minimum[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.origin()[a]) * inv_d;

            // Swap
            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            minimum: Vec3::new(0.0, 0.0, 0.0),
            maximum: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn bounding_box_sphere(center: Vec3, radius: f64) -> Aabb {
    let radius = Vec3::new(radius, radius, radius);
    Aabb::new(center - radius, center + radius)
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Vec3::new(
        box0.minimum().x.min(box1.minimum().x),
        box0.minimum().y.min(box1.minimum().y),
        box0.minimum().z.min(box1.minimum().z),
    );
    let big = Vec3::new(
        box0.maximum().x.max(box1.maximum().x),
        box0.maximum().y.max(box1.maximum().y),
        box0.maximum().z.max(box1.maximum().z),
    );
    Aabb::new(small, big)
}
