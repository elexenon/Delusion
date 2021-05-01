use nalgebra::{Vector2,Vector3,Matrix,Matrix4,Vector4,Matrix2};
use crate::transform::*;
use std::fmt::{Display, Formatter, Error};

pub static MSAA_LEVEL: usize = 4;
pub static MSAA_POS_STEP: f32 = 0.25;
pub static MSAA_SAMPLE_POS: Matrix2<Vector2<f32>> = Matrix2::new(
    Vector2::new(-MSAA_POS_STEP,-MSAA_POS_STEP),Vector2::new(MSAA_POS_STEP,MSAA_POS_STEP),
    Vector2::new(-MSAA_POS_STEP,MSAA_POS_STEP),Vector2::new(MSAA_POS_STEP,-MSAA_POS_STEP),
);

pub struct MsaaTensor {
    conv: Matrix2<Vector2<f32>>,
    mask: Vec<Vector4<bool>>,
    dept: Vec<Vector4<f32>>,
    colo: Vec<Vector4<Vector3<f32>>>,
}

impl MsaaTensor {
    pub fn new(width: usize, height: usize) -> MsaaTensor {
        let mut conv_tmp: Matrix2<Vector2<f32>> = Default::default();
        let rotate: Matrix2<f32> = rotate_matrix2d(-26.6);
        for i in 0..4 {
            conv_tmp[i] = rotate*MSAA_SAMPLE_POS[i];
        }
        MsaaTensor {
            conv: conv_tmp,
            mask: vec![Default::default();width*height],
            dept: vec![Vector4::repeat(f32::MIN);width*height],
            colo: vec![Default::default();width*height],
        }
    }

    #[inline]
    pub fn pos(&self,idx: usize) -> &Vector2<f32> { &self.conv[idx] }
    #[inline]
    pub fn set_mask(&mut self,ipixel: usize,idx: usize,flag: bool) { self.mask[ipixel][idx] = flag; }
    #[inline]
    pub fn get_mask(&mut self,ipixel: usize,idx: usize) -> bool { self.mask[ipixel][idx] }
    #[inline]
    pub fn set_dept(&mut self,ipixel: usize,idx: usize,value: f32) { self.dept[ipixel][idx] = value; }
    #[inline]
    pub fn get_dept(&mut self,ipixel: usize,idx: usize) -> f32 { self.dept[ipixel][idx] }
}

#[derive(PartialEq)]
pub enum MsaaOptions {
    Disable,
    X4,
}
impl Display for MsaaOptions {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut info: &str = "";
        match self {
            MsaaOptions::Disable => { info = "Disable" }
            MsaaOptions::X4 => { info = "4x" }
        }
        write!(f,"{}",info)
    }
}

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

