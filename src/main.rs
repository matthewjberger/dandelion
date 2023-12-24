mod map;
mod player;
mod renderer;

use crate::{
    map::{Map, Wall},
    renderer::{Renderer, RendererError},
};
use player::Player;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, Sdl, VideoSubsystem};
use std::time::Duration;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

fn init_sdl_context() -> Result<Sdl, String> {
    sdl2::init().map_err(|e| e.to_string())
}

fn create_window_and_canvas(
    video_subsystem: &VideoSubsystem,
) -> Result<sdl2::render::Canvas<sdl2::video::Window>, String> {
    video_subsystem
        .window(
            "Rust-SDL2 Demo with Double Buffer",
            WIDTH as u32,
            HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map(|window| window.into_canvas().build())
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

fn present(
    renderer: &Renderer,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), RendererError> {
    let texture_creator = canvas.texture_creator();
    let (width, height) = renderer.dimensions();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGB24, width as u32, height as u32)
        .map_err(|_| RendererError::TextureCreationError)?;

    texture
        .update(None, renderer.front_buffer_as_bytes(), width * 3)
        .map_err(|_| RendererError::TextureUpdateError)?;
    canvas
        .copy(&texture, None, None)
        .map_err(|_| RendererError::TextureCopyError)?;

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = init_sdl_context()?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
    let mut canvas = create_window_and_canvas(&video_subsystem)?;
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut player = Player::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);
    let map = create_sample_map();

    'running: loop {
        renderer.draw(&map, &player);
        renderer.swap_buffers();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => player.move_forward(1.0),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => player.move_forward(-1.0),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => player.rotate(-0.1),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => player.rotate(0.1),
                _ => {}
            }
        }

        present(&renderer, &mut canvas).map_err(|e| e.to_string())?;
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn create_sample_map() -> Map {
    let mut map = Map::new(WIDTH, HEIGHT);

    // Create walls on the outside
    for x in 0..WIDTH {
        map.set_wall(x, 0, Wall::Solid);
        map.set_wall(x, HEIGHT - 1, Wall::Solid);
    }
    for y in 0..HEIGHT {
        map.set_wall(0, y, Wall::Solid);
        map.set_wall(WIDTH - 1, y, Wall::Solid);
    }

    // Add staggered walls in the middle
    for y in (HEIGHT / 4)..(3 * HEIGHT / 4) {
        for x in (WIDTH / 4)..(3 * WIDTH / 4) {
            if (x + y) % 2 == 0 {
                map.set_wall(x, y, Wall::Brick);
            } else {
                map.set_wall(x, y, Wall::Metal);
            }
        }
    }

    map
}
