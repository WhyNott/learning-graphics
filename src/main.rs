extern crate minifb;
extern crate vecmath;
mod math;
mod gfx;


use minifb::{Key, Window, WindowOptions, MouseMode};
use gfx::colors::{Color, from_u8_rgb, from_u8_rgba};

use gfx::bitmaps::Bitmap;
use gfx::font::{Font};
use gfx::load_tga::{load_bitmap_from_tga};
use gfx::render_2d::{Polygon2D, TexturedFlat2D, Surface2D, TexturedPolygon2D, Viewport};

use gfx::render_3d::{Model, Instance, MaterialData};

use gfx::primitives::putpixel;

use math::{col_mat4_mul};

const WINDOW_WIDTH: usize = 640*2;
const WINDOW_HEIGHT: usize = 640*2;

const VIEWPORT_WIDTH: f64 = 6.0;
const VIEWPORT_HEIGHT: f64 = 6.0;


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
    //let mut glass_flat = TexturedFlat2D::new(&texture, [
    //    (WINDOW_WIDTH/2) as f64*-1.0, (WINDOW_HEIGHT/2) as f64*-1.0, 1.0]);

    
    
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

    let cube_model = Model {
        vertices: vec![
            [1.0, 1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0, 1.0],
            [1.0, -1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0, 1.0],
            [-1.0, 1.0, -1.0, 1.0],
            [-1.0, -1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0, 1.0]
        ],
        triangles: vec![
            (0, 1, 2),
            (0, 2, 3),
            (4, 0, 3),
            (4, 3, 7),
            (5, 4, 7),
            (5, 7, 6),
            (1, 5, 6),
            (1, 6, 2),
            (4, 5, 1),
            (4, 1, 0),
            (2, 6, 7),
            (2, 7, 3)
        ]
    };
    

    let mut viewport = Viewport::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        1.0,
        BACKGROUND_COLOR
    );
    
    
    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //window.set_cursor_visibility(false);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cube_instance = Instance {
        model: &cube_model,
        scale: 1.0,
        r_pitch: 0.0,
        r_yaw: 0.0,
        r_roll: 0.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        material:
        MaterialData::UV(
            &texture,
            vec![
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ]
        )
//        MaterialData::Flat(vec![
//            from_u8_rgb(255, 0, 0),
//            from_u8_rgb(255, 0, 0),
//            from_u8_rgb(0, 255, 0),
//            from_u8_rgb(0, 255, 0),
//            from_u8_rgb(0, 0, 255),
//            from_u8_rgb(0, 0, 255),
//            from_u8_rgb(255, 255, 0),
//            from_u8_rgb(255, 255, 0),
//            from_u8_rgb(230,230,250),
//            from_u8_rgb(230,230,250),
//            from_u8_rgb(0, 255, 255),
//            from_u8_rgb(0, 255, 255)
//        ])
    };
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
       
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
            let (a, b, c) = (viewport.project_vertex(polygon.a),
                         viewport.project_vertex(polygon.b),
                         viewport.project_vertex(polygon.c));
           // polygon.draw(&mut viewport, *color);
        }
        
        let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Clamp).unwrap();

        let mouse_x = (mouse_x as isize) - (WINDOW_WIDTH/2) as isize;
        let mouse_y = (mouse_y as isize) - (WINDOW_HEIGHT/2) as isize;

        
        //font.draw_str_line(&mut viewport.screen, -320+4, 320, "All systems active. Zażółć gęślą jaźń.");

        
       
        //if window.is_key_down(Key::Left) {
        //    glass_flat = glass_flat.rotate(0.1, 0.5, 0.5);
        //} else  if window.is_key_down(Key::Right) {
        //    glass_flat = glass_flat.rotate(-0.1, 0.5, 0.5);
        //}
        
        //glass_flat.draw(&mut viewport);


        if window.is_key_down(Key::Left) {
            cube_instance.r_yaw += 0.1;
        } else if window.is_key_down(Key::Right) {
            cube_instance.r_yaw -= 0.1;
        }

        if window.is_key_down(Key::Up) {
            cube_instance.r_pitch += 0.1;
        } else if window.is_key_down(Key::Down) {
            cube_instance.r_pitch -= 0.1;
        }
        
        
        cube_instance.render(&mut viewport,
                             [
                                 [1.0, 0.0, 0.0, 0.0],
                                 [0.0, 1.0, 0.0, 0.0], 
                                 [0.0, 0.0, 1.0, 0.0], 
                                 [0.0, 0.0, 3.0, 1.0], 
                                 
                             ],
        );

         let val = if let Some(v) = viewport.get_dbuff_val(mouse_x, mouse_y) {
           *v
        } else {0.0};

        font.draw_str_line(&mut viewport.screen, -320+4, 320, format!("{}", val).as_str());


      //  for x in (WINDOW_WIDTH as isize/-2)..(WINDOW_WIDTH as isize/2) {
      //      for y in (WINDOW_HEIGHT as isize/-2)..(WINDOW_HEIGHT as isize/2) {
      //          let val = if let Some(v) = viewport.get_dbuff_val(x, y) {
      //              *v
      //          } else {0.0};
      // 
      //          if val != 0.0 {
      //              putpixel(&mut viewport.screen, x, y, from_u8_rgba((val * 255.0) as u8, (val * 255.0) as u8, (val * 255.0) as u8, (val * 255.0) as u8));
      //          }
      // 
      //          
      //   }
      //  }


         //crosshair.draw_on(&mut viewport.screen, mouse_x, -mouse_y);
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        
        window
            .update_with_buffer(&(viewport.screen.data), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        
        viewport.clear_screen();
    }
}


