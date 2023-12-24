use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

const WIN_W: usize = 512; // image width
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

    let player_x = 3.456; // player x position
    let player_y = 2.345; // player y position

    'running: loop {
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

        for j in 0..MAP_H {
            for i in 0..MAP_W {
                if MAP[j].chars().nth(i).unwrap() == ' ' {
                    continue; // skip empty spaces
                }
                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                draw_rectangle(
                    &mut framebuffer,
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

        // Draw the player on the map
        draw_rectangle(
            &mut framebuffer,
            WIN_W,
            WIN_H,
            (player_x * rect_w as f32) as usize,
            (player_y * rect_h as f32) as usize,
            5,
            5,
            pack_color(255, 255, 255, 255),
        );

        // Cast rays to represent the player's view direction
        cast_rays(player_x, player_y, 1.523, &mut framebuffer, rect_w, rect_h);

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
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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

fn cast_rays(
    player_x: f32,
    player_y: f32,
    player_a: f32,
    framebuffer: &mut Vec<u32>,
    rect_w: usize,
    rect_h: usize,
) {
    let mut view_dotted = false;

    for t in (0..200).map(|t| t as f32 * 0.05) {
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
