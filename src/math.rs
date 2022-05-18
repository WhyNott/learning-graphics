pub use vecmath::{
    vec3_cast,
    vec3_sub,
    vec3_add,
    vec3_mul,
    vec3_dot,
    vec3_square_len,	
    vec3_cross,	
    vec3_scale,	
    vec3_neg,	
    vec3_len,	
    vec3_inv_len,	
    vec3_normalized,	
    vec3_normalized_sub,
    col_mat3_transform,

    col_mat4_mul,
    vec4_cast,
    vec4_sub,
    vec4_add,
    vec4_mul,
    vec4_dot,
    vec4_square_len,	
    vec4_scale,	
    vec4_neg,	
    vec4_len,	
    vec4_inv_len,	
    vec4_normalized,	
    vec4_normalized_sub,
    col_mat4_transform
};

use vecmath;

pub type Vector3 = vecmath::Vector3<f64>;
pub type Vector4 = vecmath::Vector4<f64>;

pub type Matrix3 = vecmath::Matrix3<f64>;
pub type Matrix4 = vecmath::Matrix4<f64>;
