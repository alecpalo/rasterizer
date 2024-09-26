use std::env;
use std::vec;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800; // Window width
const HEIGHT: usize = 800; // Window height

fn main() {
    // args should be three points
    let args: Vec<String> = env::args().collect();
    let t: Triangle = get_triangle(&args);

    let mut window = Window::new(
        "Triangle Rasterization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Main loop: this keeps the window open until ESC is pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_triangle(&mut buffer, &t);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn get_triangle(args: &Vec<String>) -> Triangle {
    let p1 = Point {
        x: args[1].parse().unwrap(),
        y: args[2].parse().unwrap(),
    };

    let p2 = Point {
        x: args[3].parse().unwrap(),
        y: args[4].parse().unwrap(),
    };

    let p3 = Point {
        x: args[5].parse().unwrap(),
        y: args[6].parse().unwrap(),
    };

    let t: Triangle = Triangle {
        c: 0xFFFFFF,
        v0: p1,
        v1: p2,
        v2: p3,
    };
    return t;
}

fn draw_triangle(buffer: &mut Vec<u32>, t: &Triangle) {
    draw_line(buffer, &t.v0, &t.v1, t.c);
    draw_line(buffer, &t.v1, &t.v2, t.c);
    draw_line(buffer, &t.v2, &t.v0, t.c);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };

            let mut inside = true;
            inside &= edge_function(&t.v0, &t.v1, &p);
            inside &= edge_function(&t.v1, &t.v2, &p);
            inside &= edge_function(&t.v2, &t.v0, &p);

            print!("{}\n", inside);

            if inside == true {
                set_point(buffer, &p, 0xFFADD8E6);
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u32>, p0: &Point, p1: &Point, c: u32) {
    let (mut x0, mut y0) = (p0.x as i64, p0.y as i64);
    let (x1, y1) = (p1.x as i64, p1.y as i64);

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        set_point(buffer, &Point { x: x0, y: y0 }, c); // Draw the current point

        if x0 == x1 && y0 == y1 {
            break;
        } // Break when the end point is reached

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn get_index(x: i64, y: i64) -> usize {
    let index = (WIDTH as i64 * y) + x;
    return index as usize;
}

fn set_point(buffer: &mut Vec<u32>, p: &Point, c: u32) {
    buffer[get_index(p.x, p.y)] = c;
}

struct Point {
    x: i64,
    y: i64,
}

struct Triangle {
    c: u32,
    v0: Point,
    v1: Point,
    v2: Point,
}

fn edge_function(a: &Point, b: &Point, c: &Point) -> bool {
    return (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x) >= 0;
}
