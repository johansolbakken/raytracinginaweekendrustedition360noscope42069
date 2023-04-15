pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn random_vec3() -> glm::DVec3 {
    glm::dvec3(random_f64(), random_f64(), random_f64())
}

pub fn random_vec3_range(min: f64, max: f64) -> glm::DVec3 {
    glm::dvec3(
        random_f64_range(min, max),
        random_f64_range(min, max),
        random_f64_range(min, max),
    )
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_in_unit_sphere() -> glm::DVec3 {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if glm::length(p).powf(1.0) >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn near_zero(v: &glm::DVec3) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}