use crate::math::{Vector3,  Matrix3, col_mat3_transform};
use super::primitives::{draw_filled_triangle, draw_wireframe_triangle, draw_textured_triangle};
use super::bitmaps::Bitmap;
use super::colors::{Color, from_u8_rgb};

pub trait Surface2D {
    fn m_multiply(&self, mat: Matrix3) -> Self;

    fn translate(&self, x: f64, y: f64) -> Self where Self: Sized {
        let tmat: Matrix3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [x, y, 1.0]];
        self.m_multiply(tmat)
    }

    fn scale(&self, s_x: f64, s_y: f64, p_x: f64, p_y: f64) -> Self where Self: Sized {
        let smat: Matrix3 = [
            [s_x, 0.0, 0.0],
            [0.0, s_y, 0.0],
            [p_x*(1.0 - s_x), p_y*(1.0-s_y), 1.0]];
        self.m_multiply(smat)
    }

    //note: angle is given in radians, apparently
    fn rotate(&self, angle: f64, p_x: f64, p_y: f64) -> Self where Self: Sized {
        let rmat: Matrix3 = [
            [angle.cos(), angle.sin(), 0.0],
            [-angle.sin(), angle.cos(), 0.0],
            [p_x*(1.0 - angle.cos())+p_y*angle.sin(),
             p_y*(1.0 - angle.cos())-p_x*angle.sin(),
            1.0]
        ];
        self.m_multiply(rmat)
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Polygon2D {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3
}

impl Polygon2D {
    pub fn draw(&self, buffer: &mut Bitmap, color: Color) {
        draw_filled_triangle(buffer,
                             ((self.a[0]/self.a[2]) as isize,
                              (self.a[1]/self.a[2]) as isize),
                             ((self.b[0]/self.b[2]) as isize,
                              (self.b[1]/self.b[2]) as isize),
                             ((self.c[0]/self.c[2]) as isize,
                              (self.c[1]/self.c[2]) as isize),
                             color);
        draw_wireframe_triangle(buffer,
                             ((self.a[0]/self.a[2]) as isize,
                              (self.a[1]/self.a[2]) as isize),
                             ((self.b[0]/self.b[2]) as isize,
                              (self.b[1]/self.b[2]) as isize),
                             ((self.c[0]/self.c[2]) as isize,
                              (self.c[1]/self.c[2]) as isize),
                                from_u8_rgb(0, 0, 0));
    }
          
}

impl Surface2D for Polygon2D {
    fn m_multiply(&self, mat: Matrix3) -> Polygon2D {
            Polygon2D {
            a: col_mat3_transform(mat, self.a),
            b: col_mat3_transform(mat, self.b),
            c: col_mat3_transform(mat, self.c),
        }
    }
}


pub struct TexturedPolygon2D<'bitmap>{
    pub coords: Polygon2D,
    pub texture: &'bitmap Bitmap,
    pub uv_map: Polygon2D
}
impl<'bitmap> Surface2D for TexturedPolygon2D<'bitmap> {
    fn m_multiply(&self, mat: Matrix3) -> TexturedPolygon2D<'bitmap> {
        TexturedPolygon2D {
            coords: self.coords.m_multiply(mat),
            texture: self.texture,
            uv_map: self.uv_map
        }
    }
}

impl<'bitmap> TexturedPolygon2D<'bitmap>{
    pub fn draw(&self, buffer: &mut Bitmap){
        draw_textured_triangle(
            buffer,
            ((self.coords.a[0]/self.coords.a[2]) as isize,
             (self.coords.a[1]/self.coords.a[2]) as isize),
            ((self.coords.b[0]/self.coords.b[2]) as isize,
             (self.coords.b[1]/self.coords.b[2]) as isize),
            ((self.coords.c[0]/self.coords.c[2]) as isize,
             (self.coords.c[1]/self.coords.c[2]) as isize),
            ((self.uv_map.a[0]/self.uv_map.a[2]),
             (self.uv_map.a[1]/self.uv_map.a[2])),
            ((self.uv_map.b[0]/self.uv_map.b[2]),
             (self.uv_map.b[1]/self.uv_map.b[2])),
            ((self.uv_map.c[0]/self.uv_map.c[2]),
             (self.uv_map.c[1]/self.uv_map.c[2])),
            self.texture);
    }
    
}




pub struct TexturedFlat2D<'bitmap>{
    pub a: TexturedPolygon2D<'bitmap>,
    pub b: TexturedPolygon2D<'bitmap>
}
impl<'bitmap> TexturedFlat2D<'bitmap>{
    pub fn draw(&self, buffer: &mut Bitmap){
        self.a.draw(buffer);
        self.b.draw(buffer);       
    }
}


impl<'bitmap> Surface2D for TexturedFlat2D<'bitmap> {
     fn m_multiply(&self, mat:Matrix3) -> TexturedFlat2D<'bitmap> {
        TexturedFlat2D {
            a: self.a.m_multiply(mat),
            b: self.b.m_multiply(mat),
        }
    }
}

impl<'bitmap> TexturedFlat2D <'bitmap> {
    pub fn new(texture: &'bitmap Bitmap, location: Vector3) -> TexturedFlat2D<'bitmap> {
        let uv_a = Polygon2D {
            a: [0.0, 0.0, 1.0],
            b: [1.0, 0.0, 1.0],
            c: [0.0, 1.0, 1.0],
        };
        let uv_b = Polygon2D {
            a: [1.0, 0.0, 1.0],
            b: [1.0, 1.0, 1.0],
            c: [0.0, 1.0, 1.0],
        };
        
        let coords_a = Polygon2D {
            a: location,
            b: [location[0]+(texture.width as f64), location[1], 1.0],
            c: [location[0], location[1]+(texture.width as f64), 1.0],
        };
        let coords_b = Polygon2D {
            a: [location[0]+(texture.width as f64), location[1], 1.0],
            b: [location[0]+(texture.width as f64), location[1]+(texture.width as f64), 1.0],
            c: [location[0], location[1]+(texture.width as f64), 1.0],
        };
        
        TexturedFlat2D {
            a: TexturedPolygon2D {
                coords: coords_a,
                texture,
                uv_map: uv_a
            },
            b: TexturedPolygon2D {
                coords: coords_b,
                texture,
                uv_map: uv_b
            }
        }
    }
}



