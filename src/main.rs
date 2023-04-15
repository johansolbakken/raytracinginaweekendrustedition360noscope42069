use std::fs::File;
use std::io::Write as _;

fn vec3_to_u32(vec: &glm::DVec3) -> u32 {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    return r + (g << 8) + (b << 16);
}

type Point3 = glm::DVec3;

struct Camera {}

struct Renderer {
    camera: Camera,
    pixels: Vec<u32>,
    width: usize,
    height: usize,
    accum: Vec<glm::DVec3>,
    accum_count: usize,
}

impl Renderer {
    fn new() -> Renderer {
        Renderer {
            camera: Camera {},
            pixels: vec![],
            width: 0,
            height: 0,
            accum: vec![],
            accum_count: 0,
        }
    }

    fn on_resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![0; width * height];
        self.accum = vec![glm::dvec3(0.0, 0.0, 0.0); width * height];
        self.accum_count = 0;
        self.width = width;
        self.height = height;
    }

    fn render(&mut self) {
        self.accum_count += 1;
        for j in 0..self.height {
            println!("Scanlines remaining: {}", self.height - 1 - j);
            for i in 0..self.width {
                let r = i as f64 / (self.width - 1) as f64;
                let g = j as f64 / (self.height - 1) as f64;
                let b = 0.25;

                self.accum[i + j * self.width] =
                    self.accum[i + j * self.width] + glm::dvec3(r, g, b);

                let color = self.accum[i + j * self.width] / self.accum_count as f64;
                self.pixels[i + j * self.width] = vec3_to_u32(&color);
            }
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

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut renderer = Renderer::new();
    renderer.on_resize(image_width, image_height);
    renderer.render();
    renderer.save("image.ppm");
}
