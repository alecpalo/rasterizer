use std::vec;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800; // Window width
const HEIGHT: usize = 800; // Window height

fn main() {
    // Create a window with given dimensions
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
        let p: Point = Point { x: 10, y: 10 };

        let c = 0xFFFFFF;

        set_point(&mut buffer, p, c);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn set_point(buffer: &mut Vec<u32>, p: Point, c: u32) {
    let iter = ((HEIGHT * p.y as usize) + p.x as usize) as usize;
    buffer[iter] = c;
}

struct Point {
    x: i64,
    y: i64,
}

struct Triangle {
    v0: Point,
    v1: Point,
    v2: Point,
}

fn edge_function(a: Point, b: Point, c: Point) -> i64 {
    return ((b.x - a.x) * (c.y - a.y)) - ((b.y - a.y) * (c.x - a.x));
}
