use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Instant;

const MAP: [u8; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
	1, 0, 0, 0, 0, 0, 1, 1,
	1, 0, 3, 0, 3, 0, 0, 1,
	1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 3, 0, 3, 2, 0, 1,
	1, 0, 1, 0, 0, 2, 0, 1,
	1, 0, 1, 1, 0, 2, 0, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
];

const MAP_WIDTH: isize = 8;
const MAP_HEIGHT: isize = 8;

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

fn main() {
    let ctx = sdl2::init().unwrap();
    let vid_subsystem = ctx.video().unwrap();

    let window = vid_subsystem
        .window("Raycast Demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = ctx.event_pump().unwrap();

    let mut camx = 3.5;
    let mut camy = 3.5;
    let mut cam_rotation = 0.0f64;
    let mut dt = 0.0;
    let mut speed = 0.0;
    let mut rotation_speed = 0.0;
    const FOV: f64 = 3.14159 / 3.0;

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
        camx += cam_rotation.cos() * dt * speed;
        camy += cam_rotation.sin() * dt * speed;

        let mut angle = cam_rotation - FOV / 2.0;
        for i in 0..800 {
            angle += FOV * 1.0 / 800.0;
            let ray = raycast(camx, camy, angle, 64.0);

            if ray.tile_type != 0 {
                if ray.x.floor() == ray.x {
                    canvas.set_draw_color(Color::RGB(0, 180, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                }

                let d =
                    (ray.x - camx) * (cam_rotation).cos() + (ray.y - camy) * (cam_rotation).sin();
                canvas
                    .draw_line(
                        Point::new(i, (-(1.0 / d) * 300.0 / 2.0 + 300.0) as i32),
                        Point::new(i, ((1.0 / d) * 300.0 / 2.0 + 300.0) as i32),
                    )
                    .unwrap();
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
}
