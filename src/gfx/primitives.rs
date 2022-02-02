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

//important thing about this function is that it gives you exactly one value
//d for each value i, so d's can repeat but i's cannot
//also this probably shouldn't take isize but return f64
pub fn interpolate(i0:isize, d0:isize, i1:isize, d1:isize) -> Vec<f64> {
    if i0 == i1 {
        return vec![d0 as f64];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) as f64 / (i1 - i0) as f64;
    let mut d = d0 as f64;
    for _i in i0..(i1+1) {
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

    let c : [(isize, isize); 3] = [p0, p1, p2];
    let mut lowest_point = 0;
    let mut medium_point = 1;
    let mut highest_point = 2;

    //ensures the ordering
    if c[medium_point].1 < c[lowest_point].1 {
        mem::swap(&mut medium_point, &mut lowest_point);
    }
    if c[highest_point].1 < c[lowest_point].1 {
        mem::swap(&mut highest_point, &mut lowest_point);
    }
    if c[highest_point].1 < c[medium_point].1 {
        mem::swap(&mut highest_point, &mut medium_point);
    }

    let (x0, y0) = c[lowest_point];
    let (x1, y1) = c[medium_point];
    let (x2, y2) = c[highest_point];

    //we want exactly one value of x for a value of y
    let mut x0_to_x1 = interpolate(y0, x0, y1, x1);
    let mut x1_to_x2 = interpolate(y1, x1, y2, x2);
    let mut x0_to_x2 = interpolate(y0, x0, y2, x2);

    //Concatenate the (vertically) short sides
    x0_to_x1.pop();

    x0_to_x1.append(&mut x1_to_x2);
    let x0_to_x1_to_x2 = x0_to_x1;
    
    let (x_left, x_right);
    
    //we check the middle horizontal lines to see whether the vertically
    //longest line is to the left or to the right of the other two
    let m = x0_to_x1_to_x2.len() / 2;
    if x0_to_x2.len() > 1 && x0_to_x2[m] < x0_to_x1_to_x2[m] {
        x_left = x0_to_x2;
        x_right = x0_to_x1_to_x2;
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2;
    }

    for y in y0..y2 {
        let y_index = (y - y0) as usize;
        for x in (x_left[y_index] as isize)..(x_right[y_index] as isize) {
            putpixel(buffer, x, y, color);
        }
    }  
    
}

pub fn draw_textured_triangle(buffer: &mut Bitmap,
                              p0: (isize, isize),
                              p1: (isize, isize),
                              p2: (isize, isize),
                              uv0: (f64, f64),
                              uv1: (f64, f64),
                              uv2: (f64, f64),
                              texture: &Bitmap){
    
    let c : [(isize, isize); 3] = [p0, p1, p2];
    let u : [(isize, isize); 3] =[
        ((uv0.0 * texture.width as f64) as isize,
         (uv0.1 * texture.height as f64) as isize),
        ((uv1.0 * texture.width as f64) as isize,
         (uv1.1 * texture.height as f64) as isize),
        ((uv2.0 * texture.width as f64) as isize,
         (uv2.1 * texture.height as f64) as isize),
    ];

    
    
    let mut lowest_point = 0;
    let mut medium_point = 1;
    let mut highest_point = 2;

    //ensures the ordering
    if c[medium_point].1 < c[lowest_point].1 {
        mem::swap(&mut medium_point, &mut lowest_point);
    }
    if c[highest_point].1 < c[lowest_point].1 {
        mem::swap(&mut highest_point, &mut lowest_point);
    }
    if c[highest_point].1 < c[medium_point].1 {
        mem::swap(&mut highest_point, &mut medium_point);
    }

    let (x0, y0) = c[lowest_point];
    let (x1, y1) = c[medium_point];
    let (x2, y2) = c[highest_point];

    //we want exactly one value of x for a value of y
    let mut x0_to_x1 = interpolate(y0, x0, y1, x1);
    let mut x1_to_x2 = interpolate(y1, x1, y2, x2);
    let mut x0_to_x2 = interpolate(y0, x0, y2, x2);

    //Concatenate the (vertically) short sides
    x0_to_x1.pop();

    x0_to_x1.append(&mut x1_to_x2);
    let x0_to_x1_to_x2 = x0_to_x1;
    

    //now we obtain the uv coordinates

    let (u0, v0) = u[lowest_point];
    let (u1, v1) = u[medium_point];
    let (u2, v2) = u[highest_point];

    
    let mut u0_to_u1 = interpolate(y0, u0, y1, u1);
    let mut u1_to_u2 = interpolate(y1, u1, y2, u2);
    let mut u0_to_u2 = interpolate(y0, u0, y2, u2);    

    //since we can't assume anything about the uv mapping, we need to
    //calculate the v values as well (I think?)
    let mut v0_to_v1 = interpolate(y0, v0, y1, v1);
    let mut v1_to_v2 = interpolate(y1, v1, y2, v2);
    let mut v0_to_v2 = interpolate(y0, v0, y2, v2);

    //Concatenate the (vertically) short sides
    u0_to_u1.pop();
    u0_to_u1.append(&mut u1_to_u2);
    let u0_to_u1_to_u2 = u0_to_u1;
    
    v0_to_v1.pop();
    v0_to_v1.append(&mut v1_to_v2);
    let v0_to_v1_to_v2 = v0_to_v1;

    let (x_left, x_right);

    let (u_left, u_right);
    let (v_left, v_right);
    
    //we check the middle horizontal lines to see whether the vertically
    //longest line is to the left or to the right of the other two
    let m = x0_to_x1_to_x2.len() / 2;
    if x0_to_x2.len() > 1 && x0_to_x2[m] < x0_to_x1_to_x2[m] {
        x_left = x0_to_x2;
        x_right = x0_to_x1_to_x2;

        u_left = u0_to_u2;
        u_right = u0_to_u1_to_u2;
        v_left = v0_to_v2;
        v_right = v0_to_v1_to_v2;
        
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2;

        u_left = u0_to_u1_to_u2;
        u_right = u0_to_u2;
        v_left = v0_to_v1_to_v2;
        v_right = v0_to_v2;
    }
    
    

    for y in y0..y2 {
        let y_index = (y - y0) as usize;
        //find the uv coordinates of each pixel in the y-line
        let u_coords = interpolate(x_left[y_index] as isize,
                                   u_left[y_index] as isize,
                                   x_right[y_index] as isize,
                                   u_right[y_index] as isize);
        
        let v_coords = interpolate(x_left[y_index] as isize,
                                   v_left[y_index] as isize,
                                   x_right[y_index] as isize,
                                   v_right[y_index] as isize);
        
        for x in (x_left[y_index] as isize)..(x_right[y_index] as isize) {
            //sample the uv coordinates from the texture
            let x_index = (x - x_left[y_index] as isize) as usize;
            let u = u_coords[x_index] as usize;
            let v = v_coords[x_index] as usize;
            //Note: this crashes sometimes with an out-of-bounds error sometimes - why?
            let color = texture.data[v*texture.width+u];
            putpixel(buffer, x, y, color);
        }
    }
    
    
}

