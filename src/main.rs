use camera::Camera;
use renderer::Renderer;
use scene::{Scene, Sphere, Material};

mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod vec3;

fn main() {
    //
    //  Dielectric - fun next habbajuja
    //

    let ray_tracing_in_one_weekend = true;
    let mut scene = Scene::new();

    if ray_tracing_in_one_weekend {
        scene.spheres = vec![
            Sphere::new(glm::dvec3(0.0, -100.5, -1.0), 100.0, 0),
            Sphere::new(glm::dvec3(0.0, 0.0, -1.0), 0.5, 1),
            Sphere::new(glm::dvec3(-1.0, 0.0, -1.0), 0.5, 2),
            Sphere::new(glm::dvec3(1.0, 0.0, -1.0), 0.5, 3),
        ];
        scene.materials = vec![
            Material {
                albedo: glm::dvec3(0.8, 0.8, 0.0),
                roughness: 1.0,
                metallic: 0.0,
            },
            Material {
                albedo: glm::dvec3(0.7, 0.3, 0.3),
                roughness: 1.0,
                metallic: 0.0,
            },
            Material {
                albedo: glm::dvec3(0.8, 0.8, 0.8),
                roughness: 0.0,
                metallic: 0.0,
            },
            Material {
                albedo: glm::dvec3(0.8, 0.6, 0.2),
                roughness: 0.0,
                metallic: 0.0,
            },
        ];
    }

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    let samples_per_pixel = 500;
    for i in 0..samples_per_pixel {
        let start = std::time::Instant::now();

        if ray_tracing_in_one_weekend {
            renderer.render_recurse(&camera, &scene);
        } else {
            renderer.render(&camera, &scene);
        }

        let elapsed = start.elapsed();
        println!(
            "Render {}:\t{}ms",
            i+1,
            elapsed.as_millis(),
        );
    }
    renderer.save("image.ppm");
}
