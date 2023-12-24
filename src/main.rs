use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
};
use std::{collections::HashMap, f32::consts::PI, time::Duration};

// Define an enum for wall colors
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum WallColor {
    Red,
    Green,
    Blue,
    // Add more color variants as needed
}

impl WallColor {
    // Define a method to get the color as a u32
    fn as_u32(&self) -> u32 {
        match self {
            WallColor::Red => pack_color(255, 0, 0, 255),
            WallColor::Green => pack_color(0, 255, 0, 255),
            WallColor::Blue => pack_color(0, 0, 255, 255),
            // Add color mappings for other variants
        }
    }
}

const PLAYER_FOV: f32 = PI / 3.0; // field of view
const MAX_DISTANCE: f32 = 20.0; // Adjust this value as needed
const MOVEMENT_SPEED: f32 = 0.1; // Adjust as needed
const ROTATION_SPEED: f32 = 0.05; // Adjust as needed

const WIN_W: usize = 1024; // image width
const WIN_H: usize = 512; // image height
const MAP_W: usize = 16; // map width
const MAP_H: usize = 16; // map height

const MAP: [&str; MAP_H] = [
    "0000222222220000",
    "1              0",
    "1      11111   0",
    "1     0        0",
    "0     0  1110000",
    "0     3        0",
    "0   10000      0",
    "0   0   11100  0",
    "0   0   0      0",
    "0   0   1  00000",
    "0       1      0",
    "2       1      0",
    "0       0      0",
    "0 0000000      0",
    "0              0",
    "0002222222200000",
];

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust-SDL2 Demo with Game Map", WIN_W as u32, WIN_H as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut framebuffer = vec![255; WIN_W * WIN_H]; // the image itself, initialized to white

    let mut char_colors: HashMap<char, WallColor> = HashMap::new();
    char_colors.insert('0', WallColor::Red);
    char_colors.insert('1', WallColor::Green);
    char_colors.insert('2', WallColor::Blue);

    let mut player_x = 3.456; // player x position
    let mut player_y = 2.345; // player y position
    let mut player_a = 1.523; // player view direction

    let mut overlay_active = false;

    'running: loop {
        // Get the current keyboard state
        let keyboard_state = event_pump.keyboard_state();

        move_player(keyboard_state, &mut player_a, &mut player_x, &mut player_y);

        for j in 0..WIN_H {
            for i in 0..WIN_W {
                let r = (255 * j / WIN_H) as u8;
                let g = (255 * i / WIN_W) as u8;
                let b = 0;
                framebuffer[i + j * WIN_W] = pack_color(r, g, b, 255);
            }
        }

        let rect_w = WIN_W / MAP_W;
        let rect_h = WIN_H / MAP_H;

        if overlay_active {
            draw_debug(
                rect_w,
                rect_h,
                &mut framebuffer,
                player_x,
                player_y,
                player_a,
            );
        } else {
            draw_scene(
                player_x,
                player_y,
                player_a,
                PLAYER_FOV,
                &mut framebuffer,
                rect_w,
                rect_h,
                &char_colors,
            );
        }

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(
                sdl2::pixels::PixelFormatEnum::RGB24,
                WIN_W as u32,
                WIN_H as u32,
            )
            .unwrap();

        texture
            .update(None, &framebuffer_as_bytes(&framebuffer), WIN_W * 3)
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => overlay_active = !overlay_active,

                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn move_player(
    keyboard_state: sdl2::keyboard::KeyboardState<'_>,
    player_a: &mut f32,
    player_x: &mut f32,
    player_y: &mut f32,
) {
    // Check for specific key states
    if keyboard_state.is_scancode_pressed(Scancode::A) {
        *player_a -= ROTATION_SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D) {
        *player_a += ROTATION_SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::W) {
        *player_x += player_a.cos() * MOVEMENT_SPEED;
        *player_y += player_a.sin() * MOVEMENT_SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S) {
        *player_x -= player_a.cos() * MOVEMENT_SPEED;
        *player_y -= player_a.sin() * MOVEMENT_SPEED;
    }
}

fn draw_debug(
    rect_w: usize,
    rect_h: usize,
    framebuffer: &mut Vec<u32>,
    player_x: f32,
    player_y: f32,
    player_a: f32,
) {
    draw_map_overlay(rect_w, rect_h, framebuffer);
    draw_player(framebuffer, player_x, rect_w, player_y, rect_h);
    draw_fov_sector(
        player_x,
        player_y,
        player_a,
        PLAYER_FOV,
        framebuffer,
        rect_w,
        rect_h,
        &MAP,
    );
    cast_rays(player_x, player_y, player_a, framebuffer, rect_w, rect_h);
}

fn draw_player(
    framebuffer: &mut Vec<u32>,
    player_x: f32,
    rect_w: usize,
    player_y: f32,
    rect_h: usize,
) {
    draw_rectangle(
        framebuffer,
        WIN_W,
        WIN_H,
        (player_x * rect_w as f32) as usize,
        (player_y * rect_h as f32) as usize,
        5,
        5,
        pack_color(255, 255, 255, 255),
    );
}

fn draw_map_overlay(rect_w: usize, rect_h: usize, framebuffer: &mut Vec<u32>) {
    for j in 0..MAP_H {
        for i in 0..MAP_W {
            if MAP[j].chars().nth(i).unwrap() == ' ' {
                continue; // skip empty spaces
            }
            let rect_x = i * rect_w;
            let rect_y = j * rect_h;
            draw_rectangle(
                framebuffer,
                WIN_W,
                WIN_H,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                pack_color(0, 255, 255, 255),
            );
        }
    }
}

fn pack_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + r as u32
}

fn draw_rectangle(
    img: &mut Vec<u32>,
    img_w: usize,
    img_h: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u32,
) {
    assert_eq!(img.len(), img_w * img_h);
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            assert!(cx < img_w && cy < img_h);
            img[cx + cy * img_w] = color;
        }
    }
}

fn framebuffer_as_bytes(framebuffer: &Vec<u32>) -> Vec<u8> {
    framebuffer
        .iter()
        .flat_map(|&pixel| {
            let (r, g, b, _) = unpack_color(pixel);
            vec![r, g, b]
        })
        .collect()
}

fn unpack_color(color: u32) -> (u8, u8, u8, u8) {
    let r = (color & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = ((color >> 16) & 0xFF) as u8;
    let a = ((color >> 24) & 0xFF) as u8;
    (r, g, b, a)
}

fn draw_scene(
    player_x: f32,
    player_y: f32,
    player_a: f32,
    fov: f32,
    framebuffer: &mut Vec<u32>,
    rect_w: usize,
    rect_h: usize,
    char_colors: &HashMap<char, WallColor>,
) {
    let mut current_wall_color: Option<u32> = None;

    // Define the floor color (light gray)
    let floor_color = pack_color(200, 200, 200, 255);

    for i in 0..WIN_W {
        let angle = player_a - fov / 2.0 + fov * (i as f32) / WIN_W as f32;
        let mut distance_to_wall = 0.0;

        let mut wall_hit = false;
        while !wall_hit && distance_to_wall < MAX_DISTANCE {
            distance_to_wall += 0.1;

            let cx = player_x + distance_to_wall * angle.cos();
            let cy = player_y + distance_to_wall * angle.sin();
            let pix_x = (cx * rect_w as f32) as usize;
            let pix_y = (cy * rect_h as f32) as usize;

            if pix_x >= WIN_W || pix_y >= WIN_H {
                wall_hit = true;
                continue;
            }

            if let Some(map_tile) = MAP
                .get(cy as usize)
                .and_then(|row| row.chars().nth(cx as usize))
            {
                if map_tile != ' ' {
                    wall_hit = true;

                    // Determine the color for the current wall segment
                    let current_color = char_colors
                        .get(&map_tile)
                        .map(|color| color.as_u32())
                        .unwrap_or(pack_color(255, 255, 255, 255));

                    if current_wall_color != Some(current_color) {
                        current_wall_color = Some(current_color);
                    }

                    let corrected_distance = distance_to_wall * (angle - player_a).cos();
                    let column_height = ((WIN_H as f32 / corrected_distance).round()) as usize;
                    if column_height > 0 {
                        let draw_start = (WIN_H / 2).saturating_sub(column_height / 2);
                        draw_rectangle(
                            framebuffer,
                            WIN_W,
                            WIN_H,
                            i,
                            draw_start,
                            1,
                            column_height.min(WIN_H),
                            current_color,
                        );

                        // Fill the area below the walls with the floor color
                        draw_rectangle(
                            framebuffer,
                            WIN_W,
                            WIN_H,
                            i,
                            draw_start + column_height,
                            1,
                            WIN_H - (draw_start + column_height),
                            floor_color,
                        );
                    }
                } else {
                    current_wall_color = None;
                }
            }
        }
    }
}

fn cast_rays(
    player_x: f32,
    player_y: f32,
    player_a: f32,
    framebuffer: &mut Vec<u32>,
    rect_w: usize,
    rect_h: usize,
) {
    let mut view_dotted = true;

    for t in (0..400).map(|t| t as f32 * 0.05) {
        let cx = player_x + t * player_a.cos();
        let cy = player_y + t * player_a.sin();
        let map_x = cx.floor() as usize;
        let map_y = cy.floor() as usize;

        if map_x < MAP_W && map_y < MAP_H {
            let map_tile = MAP[map_y].chars().nth(map_x).unwrap();
            if map_tile != ' ' {
                if !view_dotted {
                    // Draw the dotted line until the first non-empty tile
                    view_dotted = true;
                } else {
                    break;
                }
            }
        }

        if view_dotted {
            let pix_x = (cx * rect_w as f32) as usize;
            let pix_y = (cy * rect_h as f32) as usize;
            framebuffer[pix_x + pix_y * WIN_W] = pack_color(255, 255, 255, 255);
        }
    }
}

fn draw_fov_sector(
    player_x: f32,
    player_y: f32,
    player_a: f32,
    fov: f32,
    framebuffer: &mut Vec<u32>,
    rect_w: usize,
    rect_h: usize,
    map: &[&str; MAP_H],
) {
    for i in 0..WIN_W {
        let angle = player_a - fov / 2.0 + fov * i as f32 / WIN_W as f32;

        for t in (0..400).map(|t| t as f32 * 0.05) {
            let cx = player_x + t * angle.cos();
            let cy = player_y + t * angle.sin();
            let map_x = cx.floor() as usize;
            let map_y = cy.floor() as usize;

            if map_x < MAP_W && map_y < MAP_H {
                let map_tile = map[map_y].chars().nth(map_x).unwrap();
                if map_tile != ' ' {
                    break;
                }
            }

            let pix_x = (cx * rect_w as f32) as usize;
            let pix_y = (cy * rect_h as f32) as usize;
            framebuffer[pix_x + pix_y * WIN_W] = pack_color(255, 255, 255, 255);
        }
    }
}
