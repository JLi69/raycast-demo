use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::time::Instant;

mod bitmap;

const MAP: [u8; 64] = [
    1, 1, 2, 1, 2, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 4, 1,
    1, 0, 3, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 3, 0, 3, 2, 0, 1,
    1, 0, 1, 0, 0, 2, 0, 1,
    1, 0, 1, 1, 0, 2, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

const FLOOR: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 2, 2, 2, 2, 0, 0,
    0, 2, 0, 2, 2, 2, 2, 0,
    0, 2, 2, 2, 2, 2, 2, 0,
    0, 2, 0, 3, 0, 0, 2, 0, 
    0, 2, 0, 3, 3, 0, 2, 0,
    0, 2, 0, 0, 3, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const CEILING: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 2, 0, 0, 0, 0,
    0, 1, 0, 2, 2, 0, 0, 0,
    0, 1, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const MAP_WIDTH: isize = 8;
const MAP_HEIGHT: isize = 8;

fn get_tile(x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 || x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return 0;
    }

    MAP[(x + y * MAP_WIDTH) as usize]
}

fn get_ceil(x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 || x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return 0;
    }

    CEILING[(x + y * MAP_WIDTH) as usize]
}

fn get_floor(x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 || x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return 0;
    }

    FLOOR[(x + y * MAP_WIDTH) as usize]
}

struct Raycast {
    x: f64,
    y: f64,
    tile_type: u8,
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn raycast(startx: f64, starty: f64, angle: f64, max_dist: f64) -> Raycast {
    let mut vert = Raycast {
        x: 0.0,
        y: 0.0,
        tile_type: 0,
    };

    //Check vertical lines
    if angle.cos() > 0.0 {
        let mut rayx = startx.ceil();
        let mut rayy = (rayx - startx) * angle.tan() + starty;
        while (startx - rayx).abs() < max_dist {
            let xind = rayx as isize;
            let yind = rayy.floor() as isize;

            if xind >= 0
                && xind < MAP_WIDTH
                && yind >= 0
                && yind < MAP_HEIGHT
                && MAP[(xind + yind * MAP_WIDTH) as usize] != 0
            {
                vert = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: MAP[(xind + yind * MAP_WIDTH) as usize],
                };

                break;
            }

            rayx += 1.0;
            rayy += angle.tan();
        }
    } else if angle.cos() < 0.0 {
        let mut rayx = startx.floor();
        let mut rayy = (rayx - startx) * angle.tan() + starty;
        while (startx - rayx).abs() < max_dist {
            let xind = rayx as isize - 1;
            let yind = rayy.floor() as isize;

            if xind >= 0
                && xind < MAP_WIDTH
                && yind >= 0
                && yind < MAP_HEIGHT
                && MAP[(xind + yind * MAP_WIDTH) as usize] != 0
            {
                vert = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: MAP[(xind + yind * MAP_WIDTH) as usize],
                };
                break;
            }

            rayx -= 1.0;
            rayy -= angle.tan();
        }
    }

    let mut horiz = Raycast {
        x: 0.0,
        y: 0.0,
        tile_type: 0,
    };

    //Check horizontal lines
    if angle.sin() > 0.0 {
        let mut rayy = starty.ceil();
        let mut rayx = (rayy - starty) * 1.0 / angle.tan() + startx;
        while (starty - rayy).abs() < max_dist {
            let xind = rayx.floor() as isize;
            let yind = rayy as isize;

            if xind >= 0
                && xind < MAP_WIDTH
                && yind >= 0
                && yind < MAP_HEIGHT
                && MAP[(xind + yind * MAP_WIDTH) as usize] != 0
            {
                horiz = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: MAP[(xind + yind * MAP_WIDTH) as usize],
                };
                break;
            }

            rayy += 1.0;
            rayx += 1.0 / angle.tan();
        }
    } else if angle.sin() < 0.0 {
        let mut rayy = starty.floor();
        let mut rayx = (rayy - starty) * 1.0 / angle.tan() + startx;
        while (starty - rayy).abs() < max_dist {
            let xind = rayx.floor() as isize;
            let yind = rayy as isize - 1;

            if xind >= 0
                && xind < MAP_WIDTH
                && yind >= 0
                && yind < MAP_HEIGHT
                && MAP[(xind + yind * MAP_WIDTH) as usize] != 0
            {
                horiz = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: MAP[(xind + yind * MAP_WIDTH) as usize],
                };
                break;
            }

            rayy -= 1.0;
            rayx -= 1.0 / angle.tan();
        }
    }

    //Return the value that is closest
    if (dist(horiz.x, horiz.y, startx, starty) < dist(vert.x, vert.y, startx, starty)
        && horiz.tile_type != 0)
        || vert.tile_type == 0
    {
        return horiz;
    } else {
        return vert;
    }
}

fn main() -> Result<(), String> {
    let ctx = sdl2::init().unwrap();
    let vid_subsystem = ctx.video().unwrap();

    let window = vid_subsystem
        .window("Raycast Demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("assets/textures.png")
        .map_err(|e| e.to_string())?;
    let texture_pixels =
        bitmap::BitMap::from_png("assets/textures.png").map_err(|e| e.to_string())?;
    let mut texture_shaded = texture_creator
        .load_texture("assets/textures.png")
        .map_err(|e| e.to_string())?;
    texture_shaded.set_color_mod(255 / 8 * 5, 255 / 8 * 5, 255 / 8 * 5);
    let texture_shaded = texture_shaded; //Remove mutability

    let sprite = texture_creator
        .load_texture("assets/sprite.png")
        .map_err(|e| e.to_string())?;

    let spritex = 1.5f64;
    let spritey = 1.5f64;

    let mut floor_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::BGRA8888, 200, 75)
        .unwrap();

    let mut ceil_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::BGRA8888, 200, 75)
        .unwrap();

    let mut event_pump = ctx.event_pump().unwrap();

    let mut camx = 3.5;
    let mut camy = 3.5;
    let mut cam_rotation = 0.0f64;
    let mut dt = 0.0;
    let mut speed = 0.0;
    let mut rotation_speed = 0.0;
    const FOV: f64 = 3.14159 / 12.0 * 5.0;

    let mut depthbuffer = [9999.0f64; 200];
    'running: loop {
        let start = Instant::now();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    rotation_speed = -2.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    rotation_speed = 2.0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    rotation_speed = 0.0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    rotation_speed = 0.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    speed = 2.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    speed = -2.0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    speed = 0.0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    speed = 0.0;
                }
                _ => {}
            }
        }

        cam_rotation += dt * rotation_speed;
        while cam_rotation > 3.14159 * 2.0 {
            cam_rotation -= 3.14159 * 2.0
        }
        while cam_rotation <= 0.0 {
            cam_rotation += 3.14159 * 2.0
        }

        if (speed > 0.0 && raycast(camx, camy, cam_rotation, dt * speed).tile_type == 0)
            || (speed < 0.0
                && raycast(camx, camy, cam_rotation + 3.14159, dt * -speed).tile_type == 0)
        {
            camx += cam_rotation.cos() * dt * speed;
            camy += cam_rotation.sin() * dt * speed;
        }

        floor_texture
            .with_lock(None, |pixels: &mut [u8], pitch: usize| {
                let height = pixels.len() / pitch;

                for y in 0..height {
                    let dist = (height as f64) / y as f64;

                    for x in 0..pitch / 4 {
                        let angle = x as f64 / (pitch as f64 / 4.0) * FOV - FOV / 2.0;
                        let posx = dist * angle.tan();
                        let posy = dist;

                        let floorx =
                            posx * (-cam_rotation).sin() + posy * (-cam_rotation).cos() + camx;
                        let floory =
                            posx * (-cam_rotation).cos() - posy * (-cam_rotation).sin() + camy;

                        let tile_type = get_tile(floorx.floor() as isize, floory.floor() as isize);
                        if tile_type != 0 {
                            texture_pixels.sample(
                                floorx.fract().abs() / 4.0 + 0.25 * tile_type as f64 - 0.25,
                                0.99,
                                &mut pixels[(y * pitch + x * 4 + 1)..(y * pitch + x * 4 + 4)],
                            );
                            if floory.fract() > 0.9 || floory.fract() < 0.1 {
                                pixels[y * pitch + x * 4 + 1] /= 8;
                                pixels[y * pitch + x * 4 + 2] /= 8;
                                pixels[y * pitch + x * 4 + 3] /= 8;

                                pixels[y * pitch + x * 4 + 1] *= 5;
                                pixels[y * pitch + x * 4 + 2] *= 5;
                                pixels[y * pitch + x * 4 + 3] *= 5;
                            }

                            continue;
                        }

                        let tile_type = get_floor(floorx.floor() as isize, floory.floor() as isize);
                        texture_pixels.sample(
                            floorx.fract().abs() / 4.0 + 0.25 * (tile_type as f64 - 1.0),
                            floory.fract().abs(),
                            &mut pixels[(y * pitch + x * 4 + 1)..(y * pitch + x * 4 + 4)],
                        );

                        pixels[y * pitch + x * 4 + 1] /= 2;
                        pixels[y * pitch + x * 4 + 2] /= 2;
                        pixels[y * pitch + x * 4 + 3] /= 2;
                    }
                }
            })
            .unwrap();

        canvas
            .copy(&floor_texture, None, Rect::new(0, 300, 800, 300))
            .unwrap();

        ceil_texture
            .with_lock(None, |pixels: &mut [u8], pitch: usize| {
                let height = pixels.len() / pitch;

                for y in 0..height {
                    let dist = (height as f64) / (height - y) as f64;

                    for x in 0..pitch / 4 {
                        let angle = x as f64 / (pitch as f64 / 4.0) * FOV - FOV / 2.0;
                        let posx = dist * angle.tan();
                        let posy = dist;

                        let ceilx =
                            posx * (-cam_rotation).sin() + posy * (-cam_rotation).cos() + camx;
                        let ceily =
                            posx * (-cam_rotation).cos() - posy * (-cam_rotation).sin() + camy;

                        let tile_type = get_tile(ceilx.floor() as isize, ceily.floor() as isize);
                        if tile_type != 0 {
                            texture_pixels.sample(
                                ceilx.fract().abs() / 4.0 + 0.25 * tile_type as f64 - 0.25,
                                0.0,
                                &mut pixels[(y * pitch + x * 4 + 1)..(y * pitch + x * 4 + 4)],
                            );

                            if ceily.fract() > 0.9 || ceily.fract() < 0.1 {
                                pixels[y * pitch + x * 4 + 1] /= 8;
                                pixels[y * pitch + x * 4 + 2] /= 8;
                                pixels[y * pitch + x * 4 + 3] /= 8;

                                pixels[y * pitch + x * 4 + 1] *= 5;
                                pixels[y * pitch + x * 4 + 2] *= 5;
                                pixels[y * pitch + x * 4 + 3] *= 5;
                            }

                            continue;
                        }

                        let tile_type = get_ceil(ceilx.floor() as isize, ceily.floor() as isize);
                        texture_pixels.sample(
                            ceilx.fract().abs() / 4.0 + 0.25 * (tile_type as f64 - 1.0),
                            ceily.fract().abs(),
                            &mut pixels[(y * pitch + x * 4 + 1)..(y * pitch + x * 4 + 4)],
                        );

                        pixels[y * pitch + x * 4 + 1] /= 2;
                        pixels[y * pitch + x * 4 + 2] /= 2;
                        pixels[y * pitch + x * 4 + 3] /= 2;
                    }
                }
            })
            .unwrap();

        canvas
            .copy(&ceil_texture, None, Rect::new(0, 0, 800, 300))
            .unwrap();

        let mut angle = cam_rotation - FOV / 2.0;
        for i in 0..200 {
            let ray = raycast(camx, camy, angle, 64.0);

            if ray.tile_type != 0 {
                let d =
                    (ray.x - camx) * (cam_rotation).cos() + (ray.y - camy) * (cam_rotation).sin();
                depthbuffer[i as usize] = d;
                let pixel_pos;
                if ray.x.floor() == ray.x {
                    pixel_pos = (16.0 * ray.y.fract()) as i32 + 16 * (ray.tile_type as i32 - 1);
                    canvas
                        .copy(
                            &texture,
                            Rect::new(pixel_pos, 0, 1, 16),
                            Rect::from_center(
                                Point::new(i * 4 + 2, 300),
                                4,
                                (((1.0 / d) * 150.0).ceil() * 4.0) as u32,
                            ),
                        )
                        .unwrap();
                } else {
                    pixel_pos = (16.0 * ray.x.fract()) as i32 + 16 * (ray.tile_type as i32 - 1);
                    canvas
                        .copy(
                            &texture_shaded,
                            Rect::new(pixel_pos, 0, 1, 16),
                            Rect::from_center(
                                Point::new(i * 4 + 2, 300),
                                4,
                                (((1.0 / d) * 150.0).ceil() * 4.0) as u32,
                            ),
                        )
                        .unwrap();
                }
            }

            angle += FOV * 1.0 / 200.0;
        }

        //Draw the sprite
        {
            let sprite_trans_x = spritex - camx;
            let sprite_trans_y = spritey - camy;
            let sprite_rotated_y =
                sprite_trans_x * (-cam_rotation).cos() - sprite_trans_y * (-cam_rotation).sin();
            let sprite_rotated_x =
                sprite_trans_x * (-cam_rotation).sin() + sprite_trans_y * (-cam_rotation).cos();

            let sprite_screen_size = (400.0 / sprite_rotated_y) as u32;
            let sprite_screen_y =
                (300.0 / sprite_rotated_y + 300.0 - sprite_screen_size as f64 / 2.0) as i32;
            let sprite_screen_x =
                ((sprite_rotated_x) / ((FOV / 2.0).tan() * 2.0 * sprite_rotated_y) * 800.0) as i32
                    + 400;

            /*canvas.copy(&sprite, None, Rect::from_center(Point::new(sprite_screen_x, sprite_screen_y),
            sprite_screen_size, sprite_screen_size)).unwrap();*/

            let startx = (sprite_screen_x - sprite_screen_size as i32 / 2) / 4;
            let endx = (sprite_screen_x + sprite_screen_size as i32 / 2) / 4;
            let mut pixel_x = 0.0f64;
            for i in startx..endx {
                if i >= 0
                    && (i as usize) < depthbuffer.len()
                    && depthbuffer[i as usize] > sprite_rotated_y
                {
                    canvas
                        .copy(
                            &sprite,
                            Rect::new(pixel_x as i32, 0, 1, 64),
                            Rect::from_center(
                                Point::new(i * 4 + 2, sprite_screen_y),
                                4,
                                sprite_screen_size,
                            ),
                        )
                        .unwrap();
                }
                pixel_x += 64.0 / sprite_screen_size as f64 * 4.0;
            }
        }

        canvas.set_draw_color(Color::WHITE);
        for i in 0..MAP_HEIGHT {
            for j in 0..MAP_WIDTH {
                if MAP[(j + i * MAP_WIDTH) as usize] != 0 {
                    canvas
                        .draw_rect(Rect::new((j * 32) as i32, (i * 32) as i32, 32, 32))
                        .unwrap();
                }
            }
        }

        canvas.set_draw_color(Color::RED);
        canvas
            .draw_rect(Rect::new(
                (spritex * 32.0) as i32 - 8,
                (spritey * 32.0) as i32 - 8,
                16,
                16,
            ))
            .unwrap();

        let mut angle = cam_rotation - FOV / 2.0;
        for _ in 0..80 {
            angle += FOV * 1.0 / 80.0;
            let ray = raycast(camx, camy, angle, 64.0);

            canvas.set_draw_color(Color::WHITE);

            if ray.tile_type != 0 {
                canvas
                    .draw_line(
                        Point::new((camx * 32.0) as i32, (camy * 32.0) as i32),
                        Point::new((ray.x * 32.0) as i32, (ray.y * 32.0) as i32),
                    )
                    .unwrap();
            }
        }

        canvas.present();

        dt = start.elapsed().as_secs_f64();
    }

    Ok(())
}
