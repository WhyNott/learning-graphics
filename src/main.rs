extern crate minifb;
extern crate vecmath;
mod math;
mod gfx;


use minifb::{Key, Window, WindowOptions, MouseMode};
use math::{Vector3, Vector4, Matrix4, Matrix3, col_mat3_transform};
use gfx::colors::{Color, from_u8_rgb};
use gfx::primitives::{draw_filled_triangle, draw_wireframe_triangle};
use gfx::bitmaps::Bitmap;
use gfx::font::{Font};
use gfx::load_tga::{load_bitmap_from_tga};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

//okay, considering I have all those different kinds of polygons, I'd like to implement a traid here that makes it so that as long as the underlying datastructure implements an m_muliply method it can derive all the methods for rotation, scaling, etc. 

#[derive(Debug, Clone, Copy)]
struct Polygon {
    a: Vector4,
    b: Vector4,
    c: Vector4
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
struct Polygon2D {
    a: Vector3,
    b: Vector3,
    c: Vector3
}

impl Polygon2D {
    fn draw(&self, buffer: &mut Bitmap, color: Color) {
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


struct TexturedPolygon2D<'bitmap>{
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

struct TexturedFlat2D<'bitmap>{
    pub a: TexturedPolygon2D<'bitmap>,
    pub b: TexturedPolygon2D<'bitmap>
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
    fn new(texture: &'bitmap Bitmap, location: Vector3) -> TexturedFlat2D<'bitmap> {
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




const BACKGROUND_COLOR: Color = from_u8_rgb(255, 255, 255);



fn main() {
    
    
    let textmap = load_bitmap_from_tga("bizcat.tga").unwrap();
    
    let font = Font {
        bitmap:  textmap.clone(),
        char_width: 8,
        char_height: 16,
        chars_per_line: 16,
        num_lines: 16
    };
    
      
    
    let crosshair = load_bitmap_from_tga("crosshair.tga").unwrap();
    
    let poly1 = Polygon2D {
        a: [-200.0, -250.0, 1.0],
        b: [200.0, 50.0, 1.0],
        c: [20.0, 250.0, 1.0],
    };

    let poly2 = Polygon2D {
        a: [-200.0, -250.0, 1.0],
        b: [-200.0, 50.0, 1.0],
        c: [20.0, 250.0, 1.0],
    };
    
    let mut scene : Vec<(Polygon2D, Color)> = vec![
        (poly1, from_u8_rgb(0, 255, 0)),
        (poly2, from_u8_rgb(255, 0, 0))
    ];
   
    
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_cursor_visibility(false);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = Bitmap {
            data: vec![BACKGROUND_COLOR; WIDTH * HEIGHT],
            width: WIDTH,
            height: HEIGHT
        };
        
        
        for (polygon, _) in &mut scene{
            if window.is_key_down(Key::Down) {
                *polygon = polygon.translate(0.0, -1.0);
            } else if window.is_key_down(Key::Up) {
                *polygon = polygon.translate(0.0, 1.0);
            } else if window.is_key_down(Key::NumPadPlus) {
                *polygon = polygon.scale(1.1, 1.1, 0.5, 0.5);
            } else if window.is_key_down(Key::NumPadMinus) {
                *polygon = polygon.scale(0.9, 0.9, 0.5, 0.5);
            } else if window.is_key_down(Key::Left) {
                *polygon = polygon.rotate(0.1, 0.5, 0.5);
            } if window.is_key_down(Key::Right) {
                *polygon = polygon.rotate(-0.1, 0.5, 0.5);
            }
            
        }
        
        for (polygon, color) in &scene {
            polygon.draw(&mut buffer, *color);
        }

        let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Clamp).unwrap();

        let mouse_x = (mouse_x as isize) - (WIDTH/2) as isize;
        let mouse_y = (mouse_y as isize) - (HEIGHT/2) as isize;
        

        crosshair.draw_on(&mut buffer, mouse_x, -mouse_y);

        
        font.draw_str_line(&mut buffer, -320+4, 320, "All systems active. Zażółć gęślą jaźń.");
        

        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&(buffer.data), WIDTH, HEIGHT)
            .unwrap();
    }
}


