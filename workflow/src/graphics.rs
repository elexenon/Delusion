use nalgebra::{Vector2,Vector3,Matrix,Matrix4,Vector4};
use crate::transform::*;

pub fn bounding_box(pts: &Vector3<Vector4<f32>>,bboxmin:&mut [f32;2],bboxmax:&mut [f32;2]) {
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = bboxmin[j].min(pts[i][j]/pts[i][3]);
            bboxmax[j] = bboxmax[j].max(pts[i][j]/pts[i][3]);
        }
    }
}
#[inline(always)]
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
#[inline(always)]
pub fn interior(weights: &Vector3<f32>) -> bool {
    if weights.x<0.0 || weights.y<0.0 || weights.z<0.0 {
        return false;
    }
    true
}
#[inline(always)]
pub fn barycentric(a:&Vector4<f32>,b:&Vector4<f32>,c:&Vector4<f32>,x:f32,y:f32) -> Vector3<f32> {
    let gamma: f32 = ((b.x - a.x) * (y - a.y) - (x - a.x) * (b.y - a.y)) /
        ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y));
    let beta = (x - a.x - gamma * (c.x - a.x)) / (b.x - a.x);
    let alpha = 1.0 - beta - gamma;
    Vector3::new(alpha,beta,gamma)
}

