extern crate minifb;
extern crate vecmath;
mod math;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

type Color = u32;

use math::{Vector3};

struct Sphere {
    center: Vector3,
    radius: f64,
    color: Color
}

struct Scene {
    viewport_size: (f64, f64),
    projection_plane_d: f64,
    spheres: Vec<Sphere>
}



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


fn canvas_to_viewport(scene: &Scene, x: isize, y: isize) -> Vector3 {
    let (x, y) = (x as f64, y as f64);
    let (v_w, v_h) = scene.viewport_size;
    return [x*v_w/(WIDTH as f64), y*v_h/(HEIGHT as f64), scene.projection_plane_d];
}
use math::{vec3_sub, vec3_dot};

fn intersect_ray_sphere(O: Vector3, D: Vector3,  sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let CO = vec3_sub(O, sphere.center);

    let a = vec3_dot(D, D);
    let b = 2.0*vec3_dot(CO, D);
    let c = vec3_dot(CO, CO) - r*r;

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0*a);
    let t2 = (-b - discriminant.sqrt()) / (2.0*a);
    return (t1, t2);    
}

fn trace_ray(scene: &Scene, O: Vector3, D: Vector3, t_min: f64, t_max: f64) -> Color{
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere : Option<&Sphere> = None;
    for sphere in &scene.spheres {
        let (t1, t2) = intersect_ray_sphere(O, D, sphere);
        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(&sphere);
        }
        
        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(&sphere);
        }

    }

    if let Some(sphere) = closest_sphere {
        return sphere.color;
    } else {
        return BACKGROUND_COLOR;
    }
}


fn main() {
    let scene = Scene {
        viewport_size: (1.0, 1.0),
        projection_plane_d: 1.0,
        spheres: vec![
            Sphere {
                center: [0.0, -1.0, 3.0],
                radius: 1.0,
                color: from_u8_rgb(255, 0, 0)
            },
            Sphere {
                center: [2.0, 0.0, 4.0],
                radius: 1.0,
                color: from_u8_rgb(0, 0, 255)
            },
            Sphere {
                center: [-2.0, 0.0, 4.0],
                radius: 1.0,
                color: from_u8_rgb(0, 255, 0)
            },
        ]
    };


    
    let mut buffer: Vec<Color> = vec![ BACKGROUND_COLOR; WIDTH * HEIGHT];
    //putpixel(&mut buffer, 0, 0, from_u8_rgb(0, 255, 0));
    let O: Vector3 = [0.0, 0.0, 0.0];
    for x in -(WIDTH as isize)/2..(WIDTH as isize)/2 {
        for y in -(HEIGHT as isize)/2..(HEIGHT as isize)/2 {
            let D = canvas_to_viewport(&scene, x, y);
            let color = trace_ray(&scene, O, D, 1.0, f64::INFINITY);
            putpixel(&mut buffer, x, y, color);
        }
    }


    
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
