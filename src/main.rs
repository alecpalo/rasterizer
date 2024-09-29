use std::cmp;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::vec;

use minifb::{Key, Window, WindowOptions};
use serde::Deserialize;

const WIDTH: usize = 800; // Window width
const HEIGHT: usize = 800; // Window height

#[derive(Deserialize)]
struct Csv {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    x3: i64,
    y3: i64,
    color: u32,
}

pub fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<Triangle>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut triangles: Vec<Triangle> = vec![];

    let iter = reader.deserialize();

    for result in iter {
        let record: Csv = result?;
        println!(
            "{} {} {} {} {} {} {} \n",
            record.x1, record.y1, record.x2, record.y2, record.x3, record.y3, record.color
        );

        let p0 = Point {
            x: record.x1,
            y: record.y1,
        };

        let p1 = Point {
            x: record.x2,
            y: record.y2,
        };

        let p2 = Point {
            x: record.x3,
            y: record.y3,
        };

        if !edge_function(&p0, &p1, &p2) {
            triangles.push(Triangle {
                c: record.color,
                v0: p0,
                v1: p2,
                v2: p1,
            });
        } else {
            triangles.push(Triangle {
                c: record.color,
                v0: p0,
                v1: p1,
                v2: p2,
            });
        }
    }

    Ok(triangles)
}
fn main() {
    let ts = read_csv("triangles.csv");

    let window = Window::new(
        "Triangle Rasterization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut g = Graphics { window: window };

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let triangles = ts.unwrap();

    while g.window.is_open() && !g.window.is_key_down(Key::Escape) {
        for t in &triangles {
            g.draw_triangle(&mut buffer, t);
        }
        g.window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
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

    if !edge_function(&p1, &p2, &p3) {
        return Triangle {
            c: 0xFFFFFF,
            v0: p1,
            v1: p3,
            v2: p2,
        };
    }

    return Triangle {
        c: 0xFFFFFF,
        v0: p1,
        v1: p2,
        v2: p3,
    };
}

struct Graphics {
    window: Window,
}

impl Graphics {
    fn draw_triangle(&mut self, buffer: &mut Vec<u32>, t: &Triangle) {
        Self::draw_line(buffer, &t.v0, &t.v1, t.c);
        Self::draw_line(buffer, &t.v1, &t.v2, t.c);
        Self::draw_line(buffer, &t.v2, &t.v0, t.c);

        let y_min = cmp::min(cmp::min(t.v0.y, t.v1.y), t.v2.y);
        let y_max = cmp::max(cmp::max(t.v0.y, t.v1.y), t.v2.y);
        let x_min = cmp::min(cmp::min(t.v0.x, t.v1.x), t.v2.x);
        let x_max = cmp::max(cmp::max(t.v0.x, t.v1.x), t.v2.x);

        for y in y_min..y_max {
            for x in x_min..x_max {
                let p = Point { x: x, y: y };

                let mut inside = true;
                inside &= edge_function(&t.v0, &t.v1, &p);
                inside &= edge_function(&t.v1, &t.v2, &p);
                inside &= edge_function(&t.v2, &t.v0, &p);

                print!("{}\n", inside);

                if inside == true {
                    Self::set_point(buffer, &p, t.c);
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
            Self::set_point(buffer, &Point { x: x0, y: y0 }, c); // Draw the current point

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
        buffer[Self::get_index(p.x, p.y)] = c;
    }
}

struct Point {
    x: i64,
    y: i64,
}

pub struct Triangle {
    c: u32,
    v0: Point,
    v1: Point,
    v2: Point,
}

fn edge_function(a: &Point, b: &Point, c: &Point) -> bool {
    return (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x) >= 0;
}
