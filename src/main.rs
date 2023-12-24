use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

#[derive(Default, Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default, Clone)]
struct Renderer {
    width: usize,
    height: usize,
    front_buffer: Vec<Pixel>,
    back_buffer: Vec<Pixel>,
}

impl Renderer {
    fn new(width: usize, height: usize) -> Self {
        let total_pixels = width * height;
        let black_pixel = Pixel { r: 0, g: 0, b: 0 };
        Renderer {
            width,
            height,
            front_buffer: vec![black_pixel; total_pixels],
            back_buffer: vec![black_pixel; total_pixels],
        }
    }

    fn draw(&mut self) {
        // Clear back buffer
        for pixel in &mut self.back_buffer {
            *pixel = Pixel { r: 0, g: 0, b: 0 };
        }

        // Draw a red pixel in the middle
        let mid_point = (self.width / 2) + (self.height / 2) * self.width;
        self.back_buffer[mid_point] = Pixel { r: 255, g: 0, b: 0 };
    }

    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    fn present(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(
                PixelFormatEnum::RGB24,
                self.width as u32,
                self.height as u32,
            )
            .unwrap();

        texture
            .update(None, &self.front_buffer_as_bytes(), self.width * 3)
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
    }

    fn front_buffer_as_bytes(&self) -> Vec<u8> {
        self.front_buffer
            .iter()
            .flat_map(|p| vec![p.r, p.g, p.b])
            .collect()
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Rust-SDL2 Demo with Double Buffer",
            WIDTH as u32,
            HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    'running: loop {
        renderer.draw();
        renderer.swap_buffers();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.present(&mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
