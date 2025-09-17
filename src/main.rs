use std::env;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos()
        - k * a.cos() * b.sin() * c.cos()
        + j * a.cos() * c.sin()
        + k * a.sin() * c.sin()
        + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos()
        + k * a.sin() * c.cos()
        - j * a.sin() * b.sin() * c.sin()
        + k * a.cos() * b.sin() * c.sin()
        - i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32, _c: f32) -> f32 {
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    a: f32,
    b: f32,
    c: f32,
    width: usize,
    height: usize,
    distance_from_cam: f32,
    k1: f32,
    horizontal_offset: f32,
    zbuffer: &mut [f32],
    buffer: &mut [char],
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b, c) + distance_from_cam;

    if z <= 0.0 {
        return;
    }

    let ooz = 1.0 / z;

    let xp = (width as f32 / 2.0 + horizontal_offset + k1 * ooz * x * 2.0) as i32;
    let yp = (height as f32 / 2.0 + k1 * ooz * y) as i32;

    if xp < 0 || xp >= width as i32 || yp < 0 || yp >= height as i32 {
        return;
    }

    let idx = (xp + yp * width as i32) as usize;
    if ooz > zbuffer[idx] {
        zbuffer[idx] = ooz;
        buffer[idx] = ch;
    }
}

fn render_cube(
    cube_width: f32,
    horizontal_offset: f32,
    increment_speed: f32,
    a: f32,
    b: f32,
    c: f32,
    width: usize,
    height: usize,
    distance_from_cam: f32,
    k1: f32,
    zbuffer: &mut [f32],
    buffer: &mut [char],
) {
    let mut cube_x = -cube_width;
    while cube_x < cube_width {
        let mut cube_y = -cube_width;
        while cube_y < cube_width {
            calculate_for_surface(
                cube_x, cube_y, -cube_width, '@',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );
            calculate_for_surface(
                cube_width, cube_y, cube_x, '$',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );
            calculate_for_surface(
                -cube_width, cube_y, -cube_x, '~',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );
            calculate_for_surface(
                -cube_x, cube_y, cube_width, '#',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );
            calculate_for_surface(
                cube_x, -cube_width, -cube_y, ';',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );
            calculate_for_surface(
                cube_x, cube_width, cube_y, '+',
                a, b, c, width, height, distance_from_cam, k1, horizontal_offset,
                zbuffer, buffer,
            );

            cube_y += increment_speed;
        }
        cube_x += increment_speed;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cube_width = if args.contains(&"-s".to_string()) {
        5.0
    } else if args.contains(&"-l".to_string()) {
        20.0
    } else {
        10.0
    };

    let width: usize = 160;
    let height: usize = 44;
    let distance_from_cam: f32 = 100.0;
    let k1: f32 = 40.0;
    let increment_speed: f32 = 0.6;
    let background = ' ';

    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    print!("\x1b[2J");
    io::stdout().flush().unwrap();

    loop {
        let mut buffer = vec![background; width * height];
        let mut zbuffer = vec![0.0; width * height];

        render_cube(
            cube_width,
            0.0,
            increment_speed,
            a,
            b,
            c,
            width,
            height,
            distance_from_cam,
            k1,
            &mut zbuffer,
            &mut buffer,
        );

        print!("\x1b[H");
        for row in 0..height {
            for col in 0..width {
                print!("{}", buffer[row * width + col]);
            }
            println!();
        }
        io::stdout().flush().unwrap();

        a += 0.05;
        b += 0.05;
        c += 0.01;

        sleep(Duration::from_micros(16000));
    }
}
