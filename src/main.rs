extern crate minifb;
extern crate vecmath;
mod math;
mod gfx;


use minifb::{Key, Window, WindowOptions, MouseMode};
use gfx::colors::{Color, from_u8_rgb};

use gfx::bitmaps::Bitmap;
use gfx::font::{Font};
use gfx::load_tga::{load_bitmap_from_tga};
use gfx::render_2d::{Polygon2D, TexturedFlat2D, Surface2D, TexturedPolygon2D};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

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
    
    let texture = load_bitmap_from_tga("glass.tga").unwrap();
    let mut glass_flat = TexturedFlat2D::new(&texture, [
        (WIDTH/2) as f64*-1.0, (HEIGHT/2) as f64*-1.0, 1.0]);

    
    
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

        if window.is_key_down(Key::Left) {
            glass_flat = glass_flat.rotate(0.1, 0.5, 0.5);
        } else  if window.is_key_down(Key::Right) {
            glass_flat = glass_flat.rotate(-0.1, 0.5, 0.5);
        }
        
        glass_flat.draw(&mut buffer);


        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&(buffer.data), WIDTH, HEIGHT)
            .unwrap();
    }
}


