use camera::Camera;
use scene::Scene;
use renderer::Renderer;

mod camera;
mod ray;
mod scene;
mod utils;
mod vec3;
mod renderer;

fn main() {
    let scene = Scene::new();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    let samples_per_pixel = 100;
    for _ in 0..samples_per_pixel {
        renderer.render(&camera, &scene);
    }
    renderer.save("image.ppm");
}
