extern crate minifb;
extern crate vecmath;
mod math;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

type Color = u32;

use math::{Vector3};

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> Color {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

const BACKGROUND_COLOR: Color = from_u8_rgb(255, 255, 255);


fn putpixel(buffer: &mut Vec<Color>, x: isize, y:isize, color: Color){
    let x = (WIDTH/2) as isize + x;
    let y = (HEIGHT/2) as isize - y;
    if let Some(pixel) = buffer.get_mut((y*WIDTH as isize + x) as usize) {
        *pixel = color;
    }
}

fn interpolate(i0:isize, d0:isize, i1:isize, d1:isize) -> Vec<f64> {
    if i0 == i1 {
        return vec![d0 as f64];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) as f64 / (i1 - i0) as f64;
    let mut d = d0 as f64;
    for _i in i0..i1 {
        values.push(d);
        d = d + a;
    }
    return values;
}

fn draw_line(buffer: &mut Vec<Color>, p0: (isize, isize), p1: (isize, isize),  color: Color){
    let (x0, x1, y0, y1) : (isize, isize, isize, isize);
    
    if (p1.0-p0.0).abs() > (p1.1-p0.1).abs() {
        if p0.0 > p1.0 {
            x0 = p1.0;
            y0 = p1.1;
            x1 = p0.0;
            y1 = p0.1;
        } else {
            x0 = p0.0;
            y0 = p0.1;
            x1 = p1.0;
            y1 = p1.1;
        }

        let ys = interpolate(x0, y0, x1, y1);
        for x in x0..x1{
            putpixel(buffer, x, ys[(x - x0) as usize] as isize, color);
        }
    } else {
        if p0.1 > p1.1 {
            x0 = p1.0;
            y0 = p1.1;
            x1 = p0.0;
            y1 = p0.1;
        } else {
            x0 = p0.0;
            y0 = p0.1;
            x1 = p1.0;
            y1 = p1.1;
        }

        let xs = interpolate(y0, x0, y1, x1);
        for y in y0..y1{
            putpixel(buffer, xs[(y - y0) as usize] as isize, y, color);
        }

    }
}

fn main() {
    
    let mut buffer: Vec<Color> = vec![ BACKGROUND_COLOR; WIDTH * HEIGHT];
  
    draw_line(&mut buffer, (-200, -100), (240, 120), from_u8_rgb(0, 0, 0));
    
    draw_line(&mut buffer, (-50, -200), (60, 240), from_u8_rgb(0, 0, 0));
    
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //for i in buffer.iter_mut() {
        //    *i = 0; // write something more funny here!
        //}

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
