use camera::Camera;
use renderer::Renderer;
use scene::{Material, Scene, Sphere};

mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod vec3;
mod aabb;
mod bvh;

fn main() {
    //
    //  Dielectric - fun next habbajuja
    //

    let ray_tracing_in_one_weekend = false;
    let mut scene = Scene::new();

    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    if ray_tracing_in_one_weekend {
        scene.spheres = vec![
            Sphere::new(glm::dvec3(0.0, -100.5, -1.0), 100.0, 0),
            Sphere::new(glm::dvec3(0.0, 0.0, -1.0), 0.5, 1),
            Sphere::new(glm::dvec3(-1.0, 0.0, -1.0), 0.5, 2),
            Sphere::new(glm::dvec3(-1.0, 0.0, -1.0), 0.3, 2),
            Sphere::new(glm::dvec3(1.0, 0.0, -1.0), 0.5, 3),
        ];
        scene.materials = vec![
            Material {
                albedo: glm::dvec3(0.8, 0.8, 0.0),
                roughness: 1.0,
                metallic: 0.0,
                ..Default::default()
            },
            Material {
                albedo: glm::dvec3(0.1, 0.2, 0.5),
                roughness: 1.0,
                metallic: 0.0,
                ..Default::default()
            },
            Material {
                albedo: glm::dvec3(0.8, 0.8, 0.8),
                roughness: 0.0,
                metallic: 0.0,
                glass: true,
                refraction_index: 1.5,
                ..Default::default()
            },
            Material {
                albedo: glm::dvec3(0.8, 0.6, 0.2),
                roughness: 0.0,
                metallic: 0.0,
                ..Default::default()
            },
        ];

        scene = scene::hittable_scene();

        camera.setup(
            &glm::dvec3(13.0, 2.0, 3.0),
            &glm::dvec3(0.0, 0.0, 0.0),
            &glm::dvec3(0.0, 1.0, 0.0),
            20.0,
            0.1,
            10.0,
        );
    }

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    let samples_per_pixel = 200;
    for i in 0..samples_per_pixel {
        let start = std::time::Instant::now();

        if ray_tracing_in_one_weekend {
            renderer.render_recurse(&camera, &scene);
        } else {
            renderer.render(&camera, &scene);
        }

        let elapsed = start.elapsed();
        println!("Render {}:\t{}ms", i + 1, elapsed.as_millis(),);
    }
    renderer.save("image.ppm");
}
