use crate::{
    map::{Map, Wall},
    player::Player,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Failed to create texture")]
    TextureCreationError,
    #[error("Failed to update texture")]
    TextureUpdateError,
    #[error("Failed to copy texture to canvas")]
    TextureCopyError,
}

#[derive(Default, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Default, Clone)]
pub struct Renderer {
    width: usize,
    height: usize,
    front_buffer: Vec<Pixel>,
    back_buffer: Vec<Pixel>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let total_pixels = width * height;
        let black_pixel = Pixel { r: 0, g: 0, b: 0 };
        Renderer {
            width,
            height,
            front_buffer: vec![black_pixel; total_pixels],
            back_buffer: vec![black_pixel; total_pixels],
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    pub fn front_buffer_as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.front_buffer.as_ptr() as *const u8,
                self.front_buffer.len() * 3,
            )
        }
    }

    fn clear_back_buffer(&mut self) {
        for pixel in &mut self.back_buffer {
            *pixel = Pixel { r: 0, g: 0, b: 0 };
        }
    }

    // fn draw_red_pixel(&mut self) {
    //     let mid_point = (self.width / 2) + (self.height / 2) * self.width;
    //     self.back_buffer[mid_point] = Pixel { r: 255, g: 0, b: 0 };
    // }

    pub fn draw(&mut self, map: &Map, player: &Player) {
        let fov: f32 = std::f32::consts::PI / 3.0; // Field of view
        let ray_step: f32 = fov / self.width as f32;

        for x in 0..self.width {
            let ray_angle = player.angle - (fov / 2.0) + (x as f32 * ray_step);
            let mut distance_to_wall = 0.0;

            let eye_x = ray_angle.cos();
            let eye_y = ray_angle.sin();

            let mut hit_wall = false;

            while !hit_wall && distance_to_wall < self.width as f32 {
                distance_to_wall += 0.1; // Increase this for higher precision

                let test_x = (player.x + eye_x * distance_to_wall) as isize;
                let test_y = (player.y + eye_y * distance_to_wall) as isize;

                // Test if ray is out of bounds
                let (width, height) = map.dimensions();
                if test_x < 0 || test_x >= width as isize || test_y < 0 || test_y >= height as isize
                {
                    hit_wall = true;
                    distance_to_wall = self.width as f32;
                } else {
                    // Ray is inbounds, test for wall hit
                    if let Wall::Empty = map.get_wall(test_x as usize, test_y as usize) {
                        // No wall, continue
                    } else {
                        hit_wall = true;
                    }
                }
            }

            // Calculate distance to ceiling and floor
            let ceiling =
                (self.height as f32 / 2.0) - self.height as f32 / (distance_to_wall * 2.0);
            let floor = self.height as f32 - ceiling;

            // Render wall and floor/ceiling
            for y in 0..self.height {
                if y as f32 <= ceiling {
                    self.set_pixel(
                        x,
                        y,
                        Pixel {
                            r: 10,
                            g: 100,
                            b: 200,
                        },
                    ); // Ceiling
                } else if y as f32 > ceiling && y as f32 <= floor {
                    self.set_pixel(
                        x,
                        y,
                        Pixel {
                            r: 255,
                            g: 10,
                            b: 100,
                        },
                    ); // Wall
                } else {
                    self.set_pixel(
                        x,
                        y,
                        Pixel {
                            r: 100,
                            g: 100,
                            b: 100,
                        },
                    ); // Floor
                }
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if x < self.width && y < self.height {
            self.back_buffer[y * self.width + x] = pixel;
        }
    }

    fn draw_wall(&mut self, x: usize, y: usize, wall_type: Wall) {
        let pixel = match wall_type {
            Wall::Empty => Pixel { r: 0, g: 0, b: 0 },
            Wall::Solid => Pixel {
                r: 255,
                g: 255,
                b: 255,
            },
            Wall::Brick => Pixel {
                r: 170,
                g: 84,
                b: 60,
            },
            Wall::Metal => Pixel {
                r: 160,
                g: 160,
                b: 160,
            },
        };

        if x < self.width && y < self.height {
            self.back_buffer[y * self.width + x] = pixel;
        }
    }
}
