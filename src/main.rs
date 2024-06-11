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
use std::path::Path;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const CEILING_COLOR: Color = Color::RGB(129, 175, 201);
const FLOOR_COLOR: Color = Color::RGB(102, 79, 66);
const WALL_COLOR: Color = Color::RGB(44, 71, 34);
const TINTED_WALL_COLOR: Color = Color::RGB(30, 48, 23);
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
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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
    sdl_context.mouse().show_cursor(false);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_path = Path::new("./assets/wall.png");
    texture_path.display();
    let wall_texture = load_texture(texture_path);
    let texture_width = wall_texture.width() as f64;
    let texture_height = wall_texture.height() as f64;

    // Load floor texture
    let floor_texture_path = Path::new("./assets/floor.png");
    let floor_texture = load_texture(floor_texture_path);
    let floor_tex_width = floor_texture.width() as f64;
    let floor_tex_height = floor_texture.height() as f64;

    let mut pos_x = 22.0;
    let mut pos_y = 12.0;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;
    let move_speed = 0.4;
    let rot_speed = 0.03;

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
                    let old_dir_x = dir_x;
                    dir_x = dir_x * f64::cos(rot_speed) - dir_y * f64::sin(rot_speed);
                    dir_y = old_dir_x * f64::sin(rot_speed) + dir_y * f64::cos(rot_speed);
                    let old_plane_x = plane_x;
                    plane_x = plane_x * f64::cos(rot_speed) - plane_y * f64::sin(rot_speed);
                    plane_y = old_plane_x * f64::sin(rot_speed) + plane_y * f64::cos(rot_speed);
                }
                Keycode::D => {
                    let old_dir_x = dir_x;
                    dir_x = dir_x * f64::cos(-rot_speed) - dir_y * f64::sin(-rot_speed);
                    dir_y = old_dir_x * f64::sin(-rot_speed) + dir_y * f64::cos(-rot_speed);
                    let old_plane_x = plane_x;
                    plane_x = plane_x * f64::cos(-rot_speed) - plane_y * f64::sin(-rot_speed);
                    plane_y = old_plane_x * f64::sin(-rot_speed) + plane_y * f64::cos(-rot_speed);
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

            let color = if side == 1 {
                Color::RGB(255, 0, 0)
            } else {
                Color::RGB(128, 0, 0)
            };

            // Draw the ceiling (draw_start above the wall)
            canvas.set_draw_color(CEILING_COLOR);
            if draw_start > 0 {
                canvas
                    .draw_line(
                        sdl2::rect::Point::new(x as i32, 0),
                        sdl2::rect::Point::new(x as i32, draw_start - 1),
                    )
                    .unwrap();
            }

            // Draw the floor (draw_end below the wall)
            canvas.set_draw_color(FLOOR_COLOR);
            if draw_end < SCREEN_HEIGHT as i32 - 1 {
                canvas
                    .draw_line(
                        sdl2::rect::Point::new(x as i32, draw_end + 1),
                        sdl2::rect::Point::new(x as i32, SCREEN_HEIGHT as i32 - 1),
                    )
                    .unwrap();
            }

            // Draw the wall
            let color = if side == 1 {
                TINTED_WALL_COLOR
            } else {
                WALL_COLOR
            };

            // Calculate texture X coordinate
            let wall_x = if side == 0 {
                pos_y + perp_wall_dist * ray_dir_y
            } else {
                pos_x + perp_wall_dist * ray_dir_x
            };
            let wall_x = (wall_x * texture_width) as usize % texture_width as usize;

            // Calculate Y coordinate to sample from
            let mut tex_y = ((((draw_start as f64 + draw_end as f64) / 2.0
                - SCREEN_HEIGHT as f64 / 2.0)
                / line_height as f64)
                * texture_height as f64) as usize;

            // Adjust Y coordinate for textures
            if tex_y >= texture_height as usize {
                tex_y = texture_height as usize - 1;
            }

            // Fetch texel color
            let texel_color = wall_texture.get_pixel(wall_x as u32, tex_y as u32);

            // Draw textured wall slice
            canvas.set_draw_color(Color::RGB(texel_color[0], texel_color[1], texel_color[2]));
            canvas
                .draw_line(
                    sdl2::rect::Point::new(x as i32, draw_start),
                    sdl2::rect::Point::new(x as i32, draw_end),
                )
                .unwrap();
            // canvas.set_draw_color(color);
        }
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
