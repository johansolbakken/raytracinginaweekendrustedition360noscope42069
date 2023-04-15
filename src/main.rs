use std::fs::File;
use std::io::Write as _;

struct Camera {}

struct Renderer {
    camera: Camera,
    pixels: Vec<u32>,
    width: usize,
    height: usize,
}

impl Renderer {
    fn new() -> Renderer {
        Renderer {
            camera: Camera {},
            pixels: vec![],
            width: 0,
            height: 0,
        }
    }

    fn on_resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![0; width * height];
        self.width = width;
        self.height = height;
    }

    fn render(&mut self) {
        for j in 0..self.height {
            let y = self.height - 1 - j;
            println!("Scanlines remaining: {}", y);
            for x in 0..self.width {
                let r = x as f64 / (self.width - 1) as f64;
                let g = y as f64 / (self.height - 1) as f64;
                let b = 0.25;

                let ir = (255.999 * r) as u32;
                let ig = (255.999 * g) as u32;
                let ib = (255.999 * b) as u32;

                self.pixels[x + y * self.width] = ir + (ig << 8) + (ib << 16);
            }
        }
    }

    fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes()).unwrap();
        for pixel in &self.pixels {
            let r = (pixel >> 0) & 0xFF;
            let g = (pixel >> 8) & 0xFF;
            let b = (pixel >> 16) & 0xFF;
            let line = format!("{} {} {}\n", r, g, b);
            file.write_all(line.as_bytes()).unwrap();
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
