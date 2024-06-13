extern crate image;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::mouse::MouseUtil;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::Window;
use std::borrow::Borrow;
use std::path::Path;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const CEILING_COLOR: Color = Color::RGB(255, 30, 0);
const FLOOR_COLOR: Color = Color::RGB(70, 79, 66);

#[rustfmt::skip]
const MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn load_texture(path: &Path) -> image::RgbImage {
    let img = image::open(path).unwrap().to_rgb8();
    img
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    sdl_context.mouse().show_cursor(false);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_path = Path::new("./assets/wall.png");
    let wall_texture = load_texture(texture_path);

    // Load floor texture
    let floor_texture_path = Path::new("./assets/floor.png");
    let floor_texture = load_texture(floor_texture_path);

    let ceiling_texture_path = Path::new("./assets/ceiling.png");
    let ceiling_texture = load_texture(ceiling_texture_path);

    let mut pos_x = 22.0;
    let mut pos_y = 12.0;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;
    let move_speed = 0.1;
    let rot_speed = 0.01;

    let mut buffer: Vec<u8> = vec![0; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize];

    'running: loop {
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

        // HANDLE MOVEMENT
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        for key in keys {
            match key {
                Keycode::W => {
                    if MAP[(pos_x + dir_x * move_speed) as usize][(pos_y) as usize] == 0 {
                        pos_x += dir_x * move_speed;
                    }
                    if MAP[(pos_x) as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                        pos_y += dir_y * move_speed;
                    }
                }
                Keycode::S => {
                    if MAP[(pos_x - dir_x * move_speed) as usize][(pos_y) as usize] == 0 {
                        pos_x -= dir_x * move_speed;
                    }
                    if MAP[(pos_x) as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                        pos_y -= dir_y * move_speed;
                    }
                }
                Keycode::A => {
                    // Move the player left
                    let new_pos_x: f64 = ((pos_x - dir_y * move_speed) as f64)
                        .max(0.0)
                        .min(MAP_WIDTH as f64 - 1.0);
                    let new_pos_y = ((pos_y + dir_x * move_speed) as f64)
                        .max(0.0)
                        .min(MAP_HEIGHT as f64 - 1.0);
                    if MAP[new_pos_x as usize][(pos_y) as usize] == 0 {
                        pos_x = new_pos_x;
                    }
                    if MAP[pos_x as usize][new_pos_y as usize] == 0 {
                        pos_y = new_pos_y;
                    }
                }
                Keycode::D => {
                    // Move the player right
                    let new_pos_x = (pos_x + dir_y * move_speed)
                        .max(0.0)
                        .min(MAP_WIDTH as f64 - 1.0);
                    let new_pos_y = (pos_y - dir_x * move_speed)
                        .max(0.0)
                        .min(MAP_HEIGHT as f64 - 1.0);

                    if MAP[new_pos_x as usize][(pos_y) as usize] == 0 {
                        pos_x = new_pos_x;
                    }
                    if MAP[pos_x as usize][new_pos_y as usize] == 0 {
                        pos_y = new_pos_y;
                    }
                }
                _ => {}
            }
        }

        // Handle mouse look
        let mouse_state = event_pump.mouse_state();
        let mouse_x = mouse_state.x();
        let mouse_y = mouse_state.y();

        // Calculate the difference in mouse position from the center of the screen
        let delta_x = (mouse_x - (SCREEN_WIDTH / 2) as i32) as f64;
        let delta_y = (mouse_y - (SCREEN_HEIGHT / 2) as i32) as f64;

        // Adjust the direction vectors based on mouse movement
        let mouse_speed = 0.005;
        let old_dir_x = dir_x;
        dir_x = dir_x * f64::cos(-delta_x * mouse_speed) - dir_y * f64::sin(-delta_x * mouse_speed);
        dir_y =
            old_dir_x * f64::sin(-delta_x * mouse_speed) + dir_y * f64::cos(-delta_x * mouse_speed);
        let old_plane_x = plane_x;
        plane_x =
            plane_x * f64::cos(-delta_x * mouse_speed) - plane_y * f64::sin(-delta_x * mouse_speed);
        plane_y = old_plane_x * f64::sin(-delta_x * mouse_speed)
            + plane_y * f64::cos(-delta_x * mouse_speed);

        // Reset the mouse position to the center of the screen
        sdl_context.mouse().warp_mouse_in_window(
            canvas.window(),
            SCREEN_WIDTH as i32 / 2,
            SCREEN_HEIGHT as i32 / 2,
        );
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        buffer.fill(0);

        // FLOOR CASTING
        for y in 0..SCREEN_HEIGHT {
            // rayDir for leftmost ray (x = 0) and rightmost ray (x = w)
            let ray_dir_x0 = dir_x - plane_x;
            let ray_dir_y0 = dir_y - plane_y;
            let ray_dir_x1 = dir_x + plane_x;
            let ray_dir_y1 = dir_y + plane_y;

            // Current y position compared to the center of the screen (the horizon)
            let p = y as f64 - SCREEN_HEIGHT as f64 / 2.0;

            // Vertical position of the camera.
            let pos_z = 0.5 * SCREEN_HEIGHT as f64;

            // Horizontal distance from the camera to the floor for the current row.
            // 0.5 is the z position exactly in the middle between floor and ceiling.
            let row_distance = pos_z / p;

            // calculate the real world step vector we have to add for each x (parallel to camera plane)
            // adding step by step avoids multiplications with a weight in the inner loop
            let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / SCREEN_WIDTH as f64;
            let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / SCREEN_WIDTH as f64;

            // real world coordinates of the leftmost column. This will be updated as we step to the right.
            let mut floor_x = pos_x + row_distance * ray_dir_x0;
            let mut floor_y = pos_y + row_distance * ray_dir_y0;

            for x in 0..SCREEN_WIDTH {
                // the cell coord is simply got from the integer parts of floor_x and floor_y
                let cell_x = floor_x as i32;
                let cell_y = floor_y as i32;

                // get the texture coordinate from the fractional part
                let tx = (floor_texture.width() as f64 * (floor_x - cell_x as f64)) as u32
                    & (floor_texture.width() - 1);

                let ty = (floor_texture.height() as f64 * (floor_y - cell_y as f64)) as u32
                    & (floor_texture.height() - 1);

                floor_x += floor_step_x;
                floor_y += floor_step_y;

                // floor
                let floor_color = floor_texture.get_pixel(tx, ty);
                let buffer_index = ((y as u32 * SCREEN_WIDTH + x) * 4) as usize;

                buffer[buffer_index] = floor_color[0];
                buffer[buffer_index + 1] = floor_color[1];
                buffer[buffer_index + 2] = floor_color[2];

                // ceiling (symmetrical, at screen_height - y - 1 instead of y)
                let ceiling_color = ceiling_texture.get_pixel(tx, ty);
                let buffer_index =
                    (((SCREEN_HEIGHT - y - 1) as u32 * SCREEN_WIDTH + x) * 4) as usize;

                buffer[buffer_index] = ceiling_color[0];
                buffer[buffer_index + 1] = ceiling_color[1];
                buffer[buffer_index + 2] = ceiling_color[2];
            }
        }

        for x in 0..SCREEN_WIDTH {
            let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            let mut map_x = pos_x as i32;
            let mut map_y = pos_y as i32;

            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();
            let mut side_dist_x;
            let mut side_dist_y;

            let step_x;
            let step_y;

            let mut hit = 0;
            let mut side = 0;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - pos_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - pos_y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if MAP[map_x as usize][map_y as usize] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                (map_x as f64 - pos_x + (1 - step_x) as f64 / 2.0) / ray_dir_x
            } else {
                (map_y as f64 - pos_y + (1 - step_y) as f64 / 2.0) / ray_dir_y
            };

            let line_height = (SCREEN_HEIGHT as f64 / perp_wall_dist) as i32;
            let draw_start = (-line_height / 2 + SCREEN_HEIGHT as i32 / 2).max(0);
            let draw_end =
                (line_height / 2 + SCREEN_HEIGHT as i32 / 2).min(SCREEN_HEIGHT as i32 - 1);

            // Calculate texture X coordinate
            let mut texture_x = if side == 0 {
                pos_y + perp_wall_dist * ray_dir_y
            } else {
                pos_x + perp_wall_dist * ray_dir_x
            };

            texture_x -= texture_x.floor(); // Get the fractional part
            let texture_x = (texture_x as f64 * wall_texture.width() as f64) as i32
                % wall_texture.width() as i32;

            // Draw the wall slice with texture
            for y in draw_start..draw_end {
                let d = y * 256 - SCREEN_HEIGHT as i32 * 128 + line_height * 128;
                let tex_y = ((d * wall_texture.height() as i32) / line_height) / 256;
                let pixel = wall_texture.get_pixel(texture_x as u32, tex_y as u32);
                let buffer_index = ((y as u32 * SCREEN_WIDTH + x) * 4) as usize;

                buffer[buffer_index] = pixel[0];
                buffer[buffer_index + 1] = pixel[1];
                buffer[buffer_index + 2] = pixel[2];
                // buffer[buffer_index + 3] = 255; // Alpha channel
                // canvas.draw_point((x as i32, y)).unwrap();
            }

            // for y in draw_end..SCREEN_HEIGHT as i32 {
            //     let buffer_index = ((y as u32 * SCREEN_WIDTH + x) * 4) as usize;

            //     buffer[buffer_index] = FLOOR_COLOR.r;
            //     buffer[buffer_index + 1] = FLOOR_COLOR.g;
            //     buffer[buffer_index + 2] = FLOOR_COLOR.b;
            // }

            // FLOOR CASTING

            // for y in 0..draw_start {
            //     let buffer_index = ((y as u32 * SCREEN_WIDTH + x) * 4) as usize;

            //     buffer[buffer_index] = CEILING_COLOR.r;
            //     buffer[buffer_index + 1] = CEILING_COLOR.g;
            //     buffer[buffer_index + 2] = CEILING_COLOR.b;
            // }

            // // Draw the ceiling (draw_start above the wall)
            // canvas.set_draw_color(CEILING_COLOR);
            // if draw_start > 0 {
            //     canvas
            //         .draw_line(
            //             sdl2::rect::Point::new(x as i32, 0),
            //             sdl2::rect::Point::new(x as i32, draw_start - 1),
            //         )
            //         .unwrap();
            // }

            // // Draw the floor (draw_end below the wall)
            // canvas.set_draw_color(FLOOR_COLOR);
            // if draw_end < SCREEN_HEIGHT as i32 - 1 {
            //     canvas
            //         .draw_line(
            //             sdl2::rect::Point::new(x as i32, draw_end + 1),
            //             sdl2::rect::Point::new(x as i32, SCREEN_HEIGHT as i32 - 1),
            //         )
            //         .unwrap();
            // }
        }

        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
        texture
            .update(None, &buffer, (SCREEN_WIDTH * 4) as usize)
            .unwrap();

        // Copy the texture to the canvas
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
