use crate::math::{Vector4, Vector3, Matrix4,
                  col_mat4_transform, col_mat4_mul};
use super::render_2d::{Viewport};
use super::primitives::{draw_wireframe_triangle,
                        draw_filled_triangle,
                        draw_textured_triangle,
                        Interpolate, interpolate,
                        putpixel
};
use super::colors::{Color, from_u8_rgb};
use super::bitmaps::Bitmap;
use std::iter::zip;



pub enum MaterialData<'texture> {
    Flat(Vec<Color>),
    UV(&'texture Bitmap)
}
#[derive(Debug)]
pub struct PolygonData {
    pub vertex: [usize; 3],
    pub normal: [usize; 3],
    pub uv_coord: [usize; 3]
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vector4>,
    pub uv_map: Vec<Vector3>,
    pub vertex_normals: Vec<Vector3>,
    pub triangles: Vec<PolygonData>
}

impl Model {
    pub fn render_wireframe(&self,
                            view : &mut Viewport,
                            camera: Matrix4
    ){
        let mut projected : Vec<(isize, isize)> = Vec::with_capacity(self.vertices.len());
        for v in &self.vertices {
            projected.push(view.project_vertex_3d(col_mat4_transform(camera, *v)));
        }
        
        for t in &self.triangles {
            draw_wireframe_triangle(
                &mut view.screen,
                projected[t.vertex[0]],
                projected[t.vertex[1]],
                projected[t.vertex[2]],
                from_u8_rgb(0, 255, 0));
        }   
    }
}

use std::mem;

pub fn draw_textured_polygon(view: &mut Viewport,
                             p0: ((isize, isize), f64),
                             p1: ((isize, isize), f64),
                             p2: ((isize, isize), f64),
                              uv0: (f64, f64),
                              uv1: (f64, f64),
                              uv2: (f64, f64),
                              texture: &Bitmap){
    
    let c : [((isize, isize), f64); 3] = [p0, p1, p2];
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
    if c[medium_point].0.1 < c[lowest_point].0.1 {
        mem::swap(&mut medium_point, &mut lowest_point);
    }
    if c[highest_point].0.1 < c[lowest_point].0.1 {
        mem::swap(&mut highest_point, &mut lowest_point);
    }
    if c[highest_point].0.1 < c[medium_point].0.1 {
        mem::swap(&mut highest_point, &mut medium_point);
    }

    let ((x0, y0), z0) = c[lowest_point];
    let ((x1, y1), z1) = c[medium_point];
    let ((x2, y2), z2) = c[highest_point];

    let z0 = 1.0/z0 as f32; 
    let z1 = 1.0/z1 as f32; 
    let z2 = 1.0/z2 as f32;

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

    //divide by the z coordinates to fix projection

    let (u0, v0) = (u0 as f32 * z0, v0 as f32 * z0);
    let (u1, v1) = (u1 as f32 * z1, v1 as f32 * z1);
    let (u2, v2) = (u2 as f32 * z2, v2 as f32 * z2);
    
    let mut u0_to_u1 = interpolate(y0, u0, y1, u1);
    let mut u1_to_u2 = interpolate(y1, u1, y2, u2);
    let mut u0_to_u2 = interpolate(y0, u0, y2, u2);    

    //since we can't assume anything about the uv mapping, we need to
    //calculate the v values as well (I think?)
    let mut v0_to_v1 = interpolate(y0, v0, y1, v1);
    let mut v1_to_v2 = interpolate(y1, v1, y2, v2);
    let mut v0_to_v2 = interpolate(y0, v0, y2, v2);

    //Concatenate the (vertically) short sides
    u0_to_u1.end -= 1;
    
    let u0_to_u1_to_u2 = u0_to_u1.chain(u1_to_u2);
    
    v0_to_v1.end -= 1;
    
    let v0_to_v1_to_v2 = v0_to_v1.chain(v1_to_v2);

    let (x_left, x_right);

    let (u_left, u_right);
    let (v_left, v_right);


//we want exactly one value of z for a value of y
    let mut z0_to_z1 = interpolate(y0, z0, y1, z1);
    let mut z1_to_z2 = interpolate(y1, z1, y2, z2);
    let mut z0_to_z2 = interpolate(y0, z0, y2, z2);

    //Concatenate the (vertically) short sides
    z0_to_z1.end -= 1; //remove the last element to avoid repetition
    let m = (z0_to_z1.len() + z1_to_z2.len() / 2) as usize; 
    
    let z0_to_z1_to_z2 = z0_to_z1.chain(z1_to_z2);
    
    let (z_left, z_right);

    
    
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

        z_left = z0_to_z2.chain(Interpolate::empty());
        z_right = z0_to_z1_to_z2;
        
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2.chain(Interpolate::empty());

        u_left = u0_to_u1_to_u2;
        u_right = u0_to_u2.chain(Interpolate::empty());
        v_left = v0_to_v1_to_v2;
        v_right = v0_to_v2.chain(Interpolate::empty());

        z_left = z0_to_z1_to_z2;
        z_right = z0_to_z2.chain(Interpolate::empty());
    }
    
    
    //(y, ((xl, xr), (ul, ur)), ((vl, vr), (zl, zr)))
    //
    for (y, ((((xl, xr), (ul, ur)), (vl, vr)), (zl, zr))) in (y0..y2)
        .zip(x_left.zip(x_right)
             .zip(u_left.zip(u_right))
             .zip(v_left.zip(v_right))
             .zip(z_left.zip(z_right))
        ){
            
            //find the uv coordinates of each pixel in the y-line
            let u_coords = interpolate(xl as isize, ul, xr as isize, ur);
        
            let v_coords = interpolate(xl as isize, vl, xr as isize, vr);

            let z_coords = interpolate(xl as isize, zl, xr as isize, zr);

            
        let (xl, xr) = (xl as isize, xr as isize);
            for ((x, (u, v)), z) in (xl..xr)
                .zip(u_coords.zip(v_coords))
                .zip(z_coords)
            {
                if matches!(view.get_dbuff_val(x, y),
                            Some(dval)
                            if z as f64 > *dval) {
                
                    //sample the uv coordinates from the texture
                    let u = (u/z) as usize % texture.width;
                    let v = (v/z) as usize % texture.height;
           
                    let color = texture.data[v*texture.width+u];
                    putpixel(&mut view.screen, x, y, color);
                    view.set_dbuff_val(x, y, z as f64);
                }
        }
            
    }

}


pub fn draw_filled_polygon(view: &mut Viewport, p0: ((isize, isize), f64), p1: ((isize, isize), f64), p2: ((isize, isize), f64), color: Color){

    let c : [((isize, isize), f64); 3] = [p0, p1, p2];
    let mut lowest_point = 0;
    let mut medium_point = 1;
    let mut highest_point = 2;

    //ensures the ordering
    if c[medium_point].0.1 < c[lowest_point].0.1 {
        mem::swap(&mut medium_point, &mut lowest_point);
    }
    if c[highest_point].0.1 < c[lowest_point].0.1 {
        mem::swap(&mut highest_point, &mut lowest_point);
    }
    if c[highest_point].0.1 < c[medium_point].0.1 {
        mem::swap(&mut highest_point, &mut medium_point);
    }
    
    let ((x0, y0), z0) = c[lowest_point];
    let ((x1, y1), z1) = c[medium_point];
    let ((x2, y2), z2) = c[highest_point];

    let z0 = 1.0/z0; 
    let z1 = 1.0/z1; 
    let z2 = 1.0/z2; 

    //we want exactly one value of x for a value of y
    let mut x0_to_x1 = interpolate(y0, x0 as f32, y1, x1 as f32);
    let mut x1_to_x2 = interpolate(y1, x1 as f32, y2, x2 as f32);
    let mut x0_to_x2 = interpolate(y0, x0 as f32, y2, x2 as f32);

    //Concatenate the (vertically) short sides
    x0_to_x1.end -= 1; //remove the last element to avoid repetition
    let m = (x0_to_x1.len() + x1_to_x2.len() / 2) as usize;
    //ok, I think I get it
    //len should never be in the negative.
    
    let x0_to_x1_to_x2 = x0_to_x1.chain(x1_to_x2);
    
    let (x_left, x_right);


    //we want exactly one value of z for a value of y
    let mut z0_to_z1 = interpolate(y0, z0 as f32, y1, z1 as f32);
    let mut z1_to_z2 = interpolate(y1, z1 as f32, y2, z2 as f32);
    let mut z0_to_z2 = interpolate(y0, z0 as f32, y2, z2 as f32);

    //Concatenate the (vertically) short sides
    z0_to_z1.end -= 1; //remove the last element to avoid repetition
    let m = (z0_to_z1.len() + z1_to_z2.len() / 2) as usize; 
    
    let z0_to_z1_to_z2 = z0_to_z1.chain(z1_to_z2);
    
    let (z_left, z_right);

    
    //we check the middle horizontal lines to see whether the vertically
    //longest line is to the left or to the right of the other two
    //@TODO: there should be a faster way to obtain these numbers
    
    let (m_x0_x2, m_x0_x1_x2) = x0_to_x2.clone().zip(x0_to_x1_to_x2.clone()).nth(m).unwrap();
    //@TODO: do I really need to do this weird stuff with chaining?
    if x0_to_x2.len() > 1 && m_x0_x2 < m_x0_x1_x2 {
        x_left = x0_to_x2.chain(Interpolate::empty());
        x_right = x0_to_x1_to_x2;
        
        z_left = z0_to_z2.chain(Interpolate::empty());
        z_right = z0_to_z1_to_z2;
    } else {
        x_left = x0_to_x1_to_x2;
        x_right = x0_to_x2.chain(Interpolate::empty());
        
        z_left = z0_to_z1_to_z2;
        z_right = z0_to_z2.chain(Interpolate::empty());
    }

    
    for (y, ((xl, xr), (zl, zr))) in (y0..y2)
        .zip(x_left.zip(x_right)
             .zip(z_left.zip(z_right))
        ){
            let (xl, xr) = (xl as isize, xr as isize);
            for (x, z) in (xl..xr).zip(interpolate(xl, zl, xr, zr)) {
                if matches!(view.get_dbuff_val(x, y),
                            Some(dval)
                            if z as f64 > *dval) {
                    putpixel(&mut view.screen, x, y, color);
                    view.set_dbuff_val(x, y, z as f64);

                }
                
        }
    }  
}


pub struct Instance<'model, 'texture> {
    pub model: &'model Model,
    pub scale: f64,
    pub r_pitch: f64,
    pub r_yaw: f64,
    pub r_roll: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub material: MaterialData<'texture>
}


impl<'model, 'texture> Instance<'model, 'texture>{
    pub fn render(&self,
                  view : &mut Viewport,
                  camera: Matrix4) {
        //todo: just precompute the neccessary matrix by hand instead of doing all this stuff

        //scale
        let mut transform_matrix = [
            [self.scale, 0.0, 0.0, 0.0],
            [0.0, self.scale, 0.0, 0.0],
            [0.0, 0.0, self.scale, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];
        
        //pitch (x axis)
        transform_matrix = col_mat4_mul(
            transform_matrix,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, f64::cos(self.r_pitch), -f64::sin(self.r_pitch), 0.0 ],
                [0.0, f64::sin(self.r_pitch), f64::cos(self.r_pitch), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        
        //yaw (y axis)
        transform_matrix = col_mat4_mul(
            transform_matrix,
            [
                [f64::cos(self.r_yaw), 0.0, f64::sin(self.r_yaw), 0.0 ],
                [0.0, 1.0, 0.0, 0.0],
                [-f64::sin(self.r_yaw), 0.0,  f64::cos(self.r_yaw), 0.0 ],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );

        //roll (z axis)
        transform_matrix = col_mat4_mul(
            transform_matrix,
            [
                [f64::cos(self.r_roll), -f64::sin(self.r_roll), 0.0, 0.0 ],
     
                [f64::sin(self.r_roll), f64::cos(self.r_roll), 0.0, 0.0 ],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );

        //translation
        transform_matrix = col_mat4_mul(
            transform_matrix,
            [
                [1.0, 0.0, 0.0, self.x],
                [0.0, 1.0, 0.0, self.y],
                [0.0, 0.0, 1.0, self.z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        let mut projected : Vec<((isize, isize), f64)> = Vec::with_capacity(self.model.vertices.len());
        for v in &self.model.vertices {
            let vertex = col_mat4_transform(
                        col_mat4_mul(
                            camera,
                            transform_matrix
                        ),
                        *v
            );

            let z = vertex[2];
            projected.push(
                (view.project_vertex_3d(
                    vertex
                ), z)
            );
        }
        

        match &self.material {
            MaterialData::UV(texture) => {
                for t in &self.model.triangles{
                    draw_textured_polygon(
                        view,
                        projected[t.vertex[0]],
                        projected[t.vertex[1]],
                        projected[t.vertex[2]],
                        (self.model.uv_map[t.uv_coord[0]][0],
                         self.model.uv_map[t.uv_coord[0]][1]),
                        (self.model.uv_map[t.uv_coord[1]][0],
                         self.model.uv_map[t.uv_coord[1]][1]),
                        (self.model.uv_map[t.uv_coord[2]][0],
                         self.model.uv_map[t.uv_coord[2]][1]),
                        texture
                    )
                }
                
            },
            MaterialData::Flat(colors) => {        
                for (color, t) in zip(colors, &self.model.triangles) {
                    
                    draw_filled_polygon(
                        view,
                        projected[t.vertex[0]],
                        projected[t.vertex[1]],
                        projected[t.vertex[2]],
                        *color)
                }
            }

        }
    }


}
