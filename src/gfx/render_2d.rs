use crate::math::{Vector3, Vector4,  Matrix3, col_mat3_transform};
use super::primitives::{draw_filled_triangle, draw_wireframe_triangle, draw_textured_triangle};
use super::bitmaps::Bitmap;
use super::colors::{Color, from_u8_rgb};
use std::iter::zip;

pub struct Viewport {
    pub canvas_width: usize,
    pub canvas_height: usize,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub distance_d: f64,
    pub background_color: Color,
    pub depth_buffer: Vec<f64>,
    pub screen: Bitmap
}

impl Viewport {

    pub fn new(canvas_width: usize, canvas_height: usize, viewport_width: f64, viewport_height: f64, distance_d: f64, background_color: Color) -> Viewport {
        Viewport {
            canvas_width,
            canvas_height,
            viewport_width,
            viewport_height,
            distance_d,
            background_color,
            depth_buffer: vec![0.0; canvas_width*canvas_height], 
            screen: Bitmap {
                width: canvas_width,
                height: canvas_height,
                data: vec![background_color; canvas_width*canvas_height]
            }
        }
    }

    pub fn clear_screen(&mut self){
        for (pixel, depth) in zip(&mut self.screen.data, &mut self.depth_buffer){
            *pixel = self.background_color;
            *depth = 0.0;
        }
    }
        

    pub fn viewport_to_canvas(&self, x: f64, y:f64) -> (isize, isize){
        (
            (x/self.viewport_width * (self.canvas_width as f64)) as isize,
            (y/self.viewport_height * (self.canvas_height as f64)) as isize,
        )
    }

    pub fn project_vertex(&self, v: Vector3) -> (isize, isize){
        self.viewport_to_canvas(v[0]*self.distance_d/v[2],
                                v[1]*self.distance_d/v[2])
    }

    pub fn project_vertex_3d(&self, v: Vector4) -> (isize, isize){
        self.viewport_to_canvas(
            v[0]*self.distance_d/(v[2]*v[3]),
            v[1]*self.distance_d/(v[2]*v[3])
        )
    }

    pub fn get_dbuff_val(&self, x: isize, y: isize) -> Option<&f64> {
        if x < (self.canvas_width/2) as isize && x > (self.canvas_width/2) as isize*-1 {
            let x = (self.canvas_width/2) as isize + x;
            let y = (self.canvas_height/2) as isize - y;
            return self.depth_buffer.get((y*self.canvas_width as isize + x) as usize); 
        }
        None
    }

    pub fn set_dbuff_val(&mut self, x: isize, y: isize, val: f64) {
        if x < (self.canvas_width/2) as isize && x > (self.canvas_width/2) as isize*-1 {
            let x = (self.canvas_width/2) as isize + x;
            let y = (self.canvas_height/2) as isize - y;
            if let Some(pixel) = self.depth_buffer.get_mut((y*self.canvas_width as isize + x) as usize) {
                
                *pixel = val;
            }
        }
    }
}


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
    pub fn draw(&self, view: &mut Viewport, color: Color) {
        
        let (a, b, c) = (view.project_vertex(self.a),
                         view.project_vertex(self.b),
                         view.project_vertex(self.c));
        
        draw_filled_triangle(&mut view.screen,
                             a, b, c,
                             color);
        draw_wireframe_triangle(&mut view.screen,
                                a, b, c,
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


pub struct TexturedPolygon2D<'texture>{
    pub coords: Polygon2D,
    pub texture: &'texture Bitmap,
    pub uv_map: Polygon2D
}
impl<'texture> Surface2D for TexturedPolygon2D<'texture> {
    fn m_multiply(&self, mat: Matrix3) -> TexturedPolygon2D<'texture> {
        TexturedPolygon2D {
            coords: self.coords.m_multiply(mat),
            texture: self.texture,
            uv_map: self.uv_map
        }
    }
}

impl<'texture> TexturedPolygon2D<'texture>{
    pub fn draw(&self, view: &mut Viewport){
        let (a, b, c) = (view.project_vertex(self.coords.a),
                         view.project_vertex(self.coords.b),
                         view.project_vertex(self.coords.c));
        
        draw_textured_triangle(
            &mut view.screen,
            a, b, c,
            ((self.uv_map.a[0]/self.uv_map.a[2]),
             (self.uv_map.a[1]/self.uv_map.a[2])),
            ((self.uv_map.b[0]/self.uv_map.b[2]),
             (self.uv_map.b[1]/self.uv_map.b[2])),
            ((self.uv_map.c[0]/self.uv_map.c[2]),
             (self.uv_map.c[1]/self.uv_map.c[2])),
            self.texture);
    }
    
    
}




pub struct TexturedFlat2D<'texture>{
    pub a: TexturedPolygon2D<'texture>,
    pub b: TexturedPolygon2D<'texture>
}
impl<'texture> TexturedFlat2D<'texture>{
    pub fn draw(&self, view: &mut Viewport){
        self.a.draw(view);
        self.b.draw(view);       
    }
}


impl<'texture> Surface2D for TexturedFlat2D<'texture> {
     fn m_multiply(&self, mat:Matrix3) -> TexturedFlat2D<'texture> {
        TexturedFlat2D {
            a: self.a.m_multiply(mat),
            b: self.b.m_multiply(mat),
        }
    }
}

impl<'texture> TexturedFlat2D <'texture> {
    pub fn new(texture: &'texture Bitmap, location: Vector3) -> TexturedFlat2D<'texture> {
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



