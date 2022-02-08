use std::mem;
use super::bitmaps::{Bitmap};
use super::colors::{Color};



pub fn putpixel(buffer: &mut Bitmap, x: isize, y:isize, color: Color){
    if x < (buffer.width/2) as isize && x > (buffer.width/2) as isize*-1 {
        let x = (buffer.width/2) as isize + x;
        let y = (buffer.height/2) as isize - y;
        if let Some(pixel) = buffer.data.get_mut((y*buffer.width as isize + x) as usize) {
            *pixel = color;
        }
    }
}

//can I use the num thing to cast arbitrary types?
//honestly, interpolation seems like it would only really work if the internal represenation is a float
//so lets give it a shot

//...actually, maybe it would be better if instead of all this nonsense I just made it so that it requires f32 on the dependent variable and isize on the independent one
//seems like it would make much more sense, and be safer and more predictable.
//(and compile faster probably)
#[derive(Debug, Clone, Copy)]
pub struct Interpolate {
    a: f32,
    d: f32,
    current: isize,
    end: isize
}


impl Interpolate {
    fn len(&self) -> isize {
        self.end - self.current
    }

    fn empty() -> Interpolate {
        Interpolate{
            a: 0.0,
            d: 0.0,
            current: 0,
            end: 0
        }
    }
}



impl Iterator for Interpolate {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let d = self.d;
            self.d = self.d + self.a;
            self.current = self.current + 1;
            Some(d)
        }
    }
}


pub fn interpolate(i0:isize, d0:f32, i1:isize, d1:f32) -> Interpolate {
    Interpolate {
        a: if i0 == i1 {0.0} else {(d1 - d0) / (i1 - i0) as f32},
        d: d0,
        current: i0,
        end: i1 +1
    }
}



//important thing about this function is that it gives you exactly one value
//d for each value i, so d's can repeat but i's cannot
//also this probably shouldn't take isize but return f64
pub fn interpolate_old(i0:isize, d0:isize, i1:isize, d1:isize) -> Vec<f64> {
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

        let ys = interpolate(x0, y0 as f32, x1, y1 as f32);
        for (x, y) in (x0..x1).zip(ys){
            putpixel(buffer, x, y as isize, color);
        }
    } else {
        if p0.1 > p1.1 {
           mem::swap(&mut p0, &mut p1);
        }
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let xs = interpolate(y0, x0 as f32, y1, x1 as f32);
        for (x, y) in xs.zip(y0..y1){
            putpixel(buffer, x as isize, y, color);
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
    let mut x0_to_x1 = interpolate(y0, x0 as f32, y1, x1 as f32);
    let mut x1_to_x2 = interpolate(y1, x1 as f32, y2, x2 as f32);
    let mut x0_to_x2 = interpolate(y0, x0 as f32, y2, x2 as f32);

    //Concatenate the (vertically) short sides
    x0_to_x1.end -= 1; //remove the last element to avoid repetition
    let m = (x0_to_x1.len() + x1_to_x2.len() / 2) as usize; 
    
    let x0_to_x1_to_x2 = x0_to_x1.chain(x1_to_x2);
    
    let (x_left, x_right);
    
    //we check the middle horizontal lines to see whether the vertically
    //longest line is to the left or to the right of the other two
    //@TODO: there should be a faster way to obtain these numbers
    let (m_x0_x2, m_x0_x1_x2) = x0_to_x2.clone().zip(x0_to_x1_to_x2.clone()).nth(m).unwrap();

    //@TODO: do I really need to do this weird stuff with chaining?
    if x0_to_x2.len() > 1 && m_x0_x2 < m_x0_x1_x2 {
        x_left = x0_to_x2.chain(Interpolate::empty());
        x_right = x0_to_x1_to_x2;
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2.chain(Interpolate::empty());
    }
    
    for (y, (xl, xr)) in (y0..y2).zip(x_left.zip(x_right)) {
        let (xl, xr) = (xl as isize, xr as isize);
        for x in xl..xr {
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
    let mut x0_to_x1 = interpolate(y0, x0 as f32, y1, x1 as f32);
    let mut x1_to_x2 = interpolate(y1, x1 as f32, y2, x2 as f32);
    let mut x0_to_x2 = interpolate(y0, x0 as f32, y2, x2 as f32);

    //Concatenate the (vertically) short sides
    x0_to_x1.end -= 1; //remove the last element to avoid repetition
    let m = (x0_to_x1.len() + x1_to_x2.len() / 2) as usize;
    let x0_to_x1_to_x2 = x0_to_x1.chain(x1_to_x2);
    

    //now we obtain the uv coordinates

    let (u0, v0) = u[lowest_point];
    let (u1, v1) = u[medium_point];
    let (u2, v2) = u[highest_point];

    
    let mut u0_to_u1 = interpolate(y0, u0 as f32, y1, u1 as f32);
    let mut u1_to_u2 = interpolate(y1, u1 as f32, y2, u2 as f32);
    let mut u0_to_u2 = interpolate(y0, u0 as f32, y2, u2 as f32);    

    //since we can't assume anything about the uv mapping, we need to
    //calculate the v values as well (I think?)
    let mut v0_to_v1 = interpolate(y0, v0 as f32, y1, v1 as f32);
    let mut v1_to_v2 = interpolate(y1, v1 as f32, y2, v2 as f32);
    let mut v0_to_v2 = interpolate(y0, v0 as f32, y2, v2 as f32);

    //Concatenate the (vertically) short sides
    u0_to_u1.end -= 1;
    
    let u0_to_u1_to_u2 = u0_to_u1.chain(u1_to_u2);
    
    v0_to_v1.end -= 1;
    
    let v0_to_v1_to_v2 = v0_to_v1.chain(v1_to_v2);

    let (x_left, x_right);

    let (u_left, u_right);
    let (v_left, v_right);
    
    //we check the middle horizontal lines to see whether the vertically
    //longest line is to the left or to the right of the other two
    //@TODO: there should be a faster way to obtain these numbers
    let (m_x0_x2, m_x0_x1_x2) = x0_to_x2.clone().zip(x0_to_x1_to_x2.clone()).nth(m).unwrap();

    if x0_to_x2.len() > 1 &&  m_x0_x2 < m_x0_x1_x2 {
        x_left = x0_to_x2.chain(Interpolate::empty());;
        x_right = x0_to_x1_to_x2;

        u_left = u0_to_u2.chain(Interpolate::empty());;
        u_right = u0_to_u1_to_u2;
        v_left = v0_to_v2.chain(Interpolate::empty());;
        v_right = v0_to_v1_to_v2;
        
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2.chain(Interpolate::empty());

        u_left = u0_to_u1_to_u2;
        u_right = u0_to_u2.chain(Interpolate::empty());
        v_left = v0_to_v1_to_v2;
        v_right = v0_to_v2.chain(Interpolate::empty());
    }
    
    

    for (y, (((xl, xr), (ul, ur)), (vl, vr))) in (y0..y2)
        .zip(x_left.zip(x_right)
        .zip(u_left.zip(u_right))
        .zip(v_left.zip(v_right))) {
            
        //find the uv coordinates of each pixel in the y-line
        let u_coords = interpolate(xl as isize, ul, xr as isize, ur);
        
        let v_coords = interpolate(xl as isize, vl, xr as isize, vr);
        let (xl, xr) = (xl as isize, xr as isize);
        for (x, (u, v)) in (xl..xr).zip(u_coords.zip(v_coords)) {
            //sample the uv coordinates from the texture
            let u = u as usize % texture.width;
            let v = v as usize % texture.height;
           
            let color = texture.data[v*texture.width+u];
            putpixel(buffer, x, y, color);
        }
            
    }

}


