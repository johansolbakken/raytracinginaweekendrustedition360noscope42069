use camera::Camera;
use renderer::Renderer;
use scene::Scene;

mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod vec3;

fn main() {
    let scene = Scene::new();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1024;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut camera = Camera::new();
    camera.on_resize(image_width, image_height);

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    let samples_per_pixel = 5;
    for _ in 0..samples_per_pixel {
        renderer.render(&camera, &scene);
    }
    renderer.save("image.ppm");
}
