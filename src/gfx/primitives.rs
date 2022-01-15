use std::mem;

use super::bitmaps::{Bitmap};
use super::colors::{Color};



pub fn putpixel(buffer: &mut Bitmap, x: isize, y:isize, color: Color){
    let x = (buffer.width/2) as isize + x;
    let y = (buffer.height/2) as isize - y;
    if let Some(pixel) = buffer.data.get_mut((y*buffer.width as isize + x) as usize) {
        *pixel = color;
    }
}

pub fn interpolate(i0:isize, d0:isize, i1:isize, d1:isize) -> Vec<f64> {
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


pub fn draw_line(buffer: &mut Bitmap, p0: (isize, isize), p1: (isize, isize),  color: Color){
    let mut p0 = p0;
    let mut p1 = p1;
    
    if (p1.0-p0.0).abs() > (p1.1-p0.1).abs() {
        if p0.0 > p1.0 {
            mem::swap(&mut p0, &mut p1);
        }
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let ys = interpolate(x0, y0, x1, y1);
        for x in x0..x1{
            putpixel(buffer, x, ys[(x - x0) as usize] as isize, color);
        }
    } else {
        if p0.1 > p1.1 {
           mem::swap(&mut p0, &mut p1);
        }
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let xs = interpolate(y0, x0, y1, x1);
        for y in y0..y1{
            putpixel(buffer, xs[(y - y0) as usize] as isize, y, color);
        }

    }
}

pub fn draw_wireframe_triangle(buffer: &mut Bitmap, p0: (isize, isize), p1: (isize, isize), p2: (isize, isize), color: Color){
    draw_line(buffer, p0, p1, color);
    draw_line(buffer, p1, p2, color);
    draw_line(buffer, p2, p0, color);
}

pub fn draw_filled_triangle(buffer: &mut Bitmap, p0: (isize, isize), p1: (isize, isize), p2: (isize, isize), color: Color){
    let mut p0 = p0;
    let mut p1 = p1;
    let mut p2 = p2;
    
    if p1.1 < p0.1 {mem::swap(&mut p1, &mut p0);}
    if p2.1 < p0.1 {mem::swap(&mut p2, &mut p0);}
    if p2.1 < p1.1 {mem::swap(&mut p2, &mut p1);}
    
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let mut x01 = interpolate(y0, x0, y1, x1);
    let mut x12 = interpolate(y1, x1, y2, x2);
    let mut x02 = interpolate(y0, x0, y2, x2);

    // Concatenate the short sides
   // x01.pop();
    x01.append(&mut x12);
    let x012 = x01;

    let m = x012.len() / 2;
    let (x_left, x_right);

    if x02.len() > 1 && x02[m] < x012[m] {
        x_left = x02;
        x_right = x012;
    } else {
        x_left = x012;
        x_right = x02;
    }

    for y in y0..y2 {
        for x in (x_left[(y - y0) as usize] as isize)..(x_right[(y - y0) as usize] as isize) {
            putpixel(buffer, x, y, color);
        }
    }
    
}
